use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    Repository,
    commands::{checkout, rebase::create_conflict_blob},
    merge::{abort_merge, finish_merge, merge_or_rebase_in_progress, start_merge},
    objects::{
        CommitObject, RGitObject, RGitObjectTypes, create_tree_object_from_staging_tree, get_commit_tree_as_map, get_tree_as_map, instanciate_tree_files
    },
    staging::StagingTree,
    utils::find_current_repo,
};

pub fn cmd_merge(branch_name: &String, abort: bool) {
    let mut repo = match find_current_repo() {
        Some(r) => r,
        None => {
            println!("Diretório não está dentro um repositório minigit");
            return;
        }
    };

    if abort {
        abort_merge(&mut repo);
        return;
    }

    match execute_merge(&mut repo, branch_name) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn execute_merge(repo: &mut Repository, branch_name: &String) -> Result<(), String> {
    if merge_or_rebase_in_progress(repo) {
        return Err(
            "Já existe um merge ou rebase em progresso. Use --abort ou resolva os conflitos."
                .to_string(),
        );
    }

    let current_head_hash = repo.resolve_head();
    if current_head_hash.is_empty() {
        return Err("Nada para fazer merge, repositório vazio.".to_string());
    }

    let target_branch_path = repo
        .minigitdir
        .join("refs")
        .join("heads")
        .join(&branch_name)
        .join("index");
    if !target_branch_path.exists() {
        return Err(format!("Branch {} não existe.", branch_name));
    }

    let target_hash = std::fs::read_to_string(target_branch_path)
        .map_err(|_| "Não foi possível ler a referência da branch alvo.")?
        .trim()
        .to_string();

    if current_head_hash == target_hash {
        println!("Branch já atualizada.");
        return Ok(());
    }

    // Tentar realizar o fast-forward merge
    if is_ancestor(&repo, &current_head_hash, &target_hash) {
        repo.update_curr_branch(&target_hash);
        repo.clear_worktree();

        let target_object = repo
            .get_object(&target_hash)
            .ok_or("Objeto da branch alvo não encontrado.")?;
        checkout::instanciate_commit(target_object, repo);

        println!(
            "Merge concluído. Branch {} atualizada para {}.",
            repo.get_head(),
            target_hash
        );
        return Ok(());
    }

    // Tentar realizar o three-way merge
    start_merge(repo);
    std::fs::write(&repo.merge_head_path, &target_hash).expect("Erro ao escrever MERGE_HEAD");

    let common_ancestor_hash = find_common_ancestor(repo, &current_head_hash, &target_hash)
        .ok_or("Erro: Sem ancestral comum entre branches, histórias desconexas.")?;

    println!("Ancestral comum encontrado: {:?}", common_ancestor_hash);

    let head_commit_obj: CommitObject = match repo.get_object(&current_head_hash) {
        Some(RGitObjectTypes::Commit(c)) => c,
        _ => return Err("HEAD atual não aponta para um commit válido".to_string()),
    };

    let target_commit_obj: CommitObject = match repo.get_object(&target_hash) {
        Some(RGitObjectTypes::Commit(c)) => c,
        _ => return Err("Branch alvo não aponta para um commit válido".to_string()),
    };

    let base_commit_obj: CommitObject = match repo.get_object(&common_ancestor_hash) {
        Some(RGitObjectTypes::Commit(c)) => c,
        _ => return Err("Ancestral comum inválido".to_string()),
    };

    let (merge_tree_id, conflicts) =
        create_three_way_merge_tree(repo, &base_commit_obj, &head_commit_obj, &target_commit_obj);

    let merge_tree_obj = repo.get_object(&merge_tree_id).expect("Tree criada sumiu");

    if let RGitObjectTypes::Tree(tree) = merge_tree_obj {
        repo.clear_worktree();
        instanciate_tree_files(repo, &tree);

        if !conflicts.is_empty() {
            println!("CONFLITOS DETECTADOS!");
            for file in &conflicts {
                println!("CONFLICT (content): Merge conflict in {}", file);
            }

            let all_files = get_tree_as_map(repo, &tree);
            let safe_files: Vec<PathBuf> = all_files.keys()
                .filter(|path| !conflicts.contains(*path))
                .map(|p| PathBuf::from(p))
                .collect();

            repo.add_files(safe_files);

            println!("\nMerge automático falhou; conserte os conflitos e faça o commit do resultado.");
            return Ok(());
        }
    }

    let msg = format!("Merge branch '{}' into HEAD", branch_name);
    let author = format!("{} <{}>", repo.config.get_username(), repo.config.get_email());
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();

    let merge_commit = CommitObject {
        tree: merge_tree_id,
        parent: vec![current_head_hash.clone(), target_hash.clone()],
        author: author,
        message: msg,
        timestamp: now,
    };

    repo.create_object(&merge_commit);
    repo.update_curr_branch(&merge_commit.hash());
    
    finish_merge(repo);

    println!("Merge commit criado: {}", merge_commit.hash());
    return Ok(());
}

/// Verifica se 'possible_ancestor' está na história de 'descendant'
fn is_ancestor(repo: &Repository, possible_ancestor: &String, descendant: &String) -> bool {
    let mut fila = vec![descendant.clone()];

    while let Some(current_hash) = fila.pop() {
        if &current_hash == possible_ancestor {
            return true;
        }

        if let Some(RGitObjectTypes::Commit(commit)) = repo.get_object(&current_hash) {
            for parent in commit.parent {
                fila.push(parent);
            }
        }
    }

    return false;
}

/// Encontra o ancestral comum mais recente entre dois commits
fn find_common_ancestor(repo: &Repository, commit_a: &String, commit_b: &String) -> Option<String> {
    let history_a = repo.get_commit_history_from_commit(commit_a);
    let history_b = repo.get_commit_history_from_commit(commit_b);

    for ca in &history_a {
        for cb in &history_b {
            if ca.hash() == cb.hash() {
                return Some(ca.hash());
            }
        }
    }

    None
}

fn create_three_way_merge_tree(
    repo: &mut Repository,
    base: &CommitObject,
    ours: &CommitObject,
    theirs: &CommitObject,
) -> (String, HashSet<String>) {
    let map_base = get_commit_tree_as_map(repo, base);
    let map_ours = get_commit_tree_as_map(repo, ours);
    let map_theirs = get_commit_tree_as_map(repo, theirs);

    let mut all_paths: HashSet<&String> = HashSet::new();
    all_paths.extend(map_base.keys());
    all_paths.extend(map_ours.keys());
    all_paths.extend(map_theirs.keys());

    let mut final_map: HashMap<String, String> = HashMap::new();
    let mut conflicts: HashSet<String> = HashSet::new();

    for path in all_paths {
        let h_base = map_base.get(path);
        let h_ours = map_ours.get(path);
        let h_theirs = map_theirs.get(path);

        if h_ours == h_theirs {
            // Caso 1: Ambos iguais ou ambos deletados
            if let Some(h) = h_ours {
                final_map.insert(path.clone(), h.clone());
            }
        } else if h_ours == h_base {
            // Caso 2: Nós (ours) não mexemos, eles (theirs) mexeram, pega a versão deles
            if let Some(h) = h_theirs {
                final_map.insert(path.clone(), h.clone());
            }
        } else if h_theirs == h_base {
            // Caso 3: Eles (theirs) não mexeram, nós (ours) mexemos, pega a nossa versão
            if let Some(h) = h_ours {
                final_map.insert(path.clone(), h.clone());
            }
        } else {
            // Caso 4: Conflitos, ambos mexeram diferentes
            conflicts.insert(path.clone());

            let content_ours = if let Some(hash) = h_ours {
                match repo
                    .get_object(hash)
                    .expect("Objeto blob não encontrado no banco")
                {
                    RGitObjectTypes::Blob(blob) => blob.content,
                    _ => panic!("Objeto esperado é um blob"),
                }
            } else {
                Vec::new() // Arquivo deletado
            };

            let content_theirs = if let Some(hash) = h_theirs {
                match repo
                    .get_object(hash)
                    .expect("Objeto blob não encontrado no banco")
                {
                    RGitObjectTypes::Blob(blob) => blob.content,
                    _ => panic!("Objeto esperado é um blob"),
                }
            } else {
                Vec::new() // Arquivo deletado
            };

            let conflict_hash = create_conflict_blob(repo, content_ours, content_theirs);
            final_map.insert(path.clone(), conflict_hash);
        }
    }
    let mut staging_tree = StagingTree::Fork(HashMap::new());
    for (path_str, hash) in final_map {
        staging_tree.insert(hash, PathBuf::from_str(&path_str).unwrap());
    }

    let tree_id = create_tree_object_from_staging_tree(&staging_tree, repo);
    return (tree_id, conflicts);
}
