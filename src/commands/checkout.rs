use crate::{Repository, checks::{ensure_no_merge_in_progress, ensure_no_rebase_in_progress}, objects::{CommitObject, RGitObjectTypes, tree}, staging::{self, StagingEntry}, status::get_uncommited_files, utils::{find_current_repo, is_valid_sha1, resolve_head_or_branch_name}};

pub fn cmd_checkout(reference_to_commit: &String) {
    match execute_checkout(reference_to_commit) {
        Ok(..) => {
            println!("Indo para o commit {}", reference_to_commit)
        },
        Err(err) => {
            println!("Erro: {}.", err);
        }
    }
}

fn execute_checkout(reference_to_commit: &String) -> Result<(), String> {
    let mut repository = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    ensure_no_rebase_in_progress(&repository)?;
    ensure_no_merge_in_progress(&repository)?;
    prompt_uncommited_changes(&repository)?;
    
    let is_commit_id = is_valid_sha1(&reference_to_commit);

    if is_commit_id {
        repository.clear_worktree();

        let RGitObjectTypes::Commit(object) = repository
            .get_object(&reference_to_commit)
            .ok_or("Não é um commit reconhecido pelo minigit")?
            else {
                return Err("Não é um commit reconhecido pelo minigit".to_string());
            };

        instanciate_commit(object, &mut repository);
        repository.change_head(reference_to_commit);
    } else {
        let commit_id = resolve_head_or_branch_name(&reference_to_commit, &repository)
            .ok_or("Referência não existe")?;

        repository.clear_worktree();
        repository.change_head(reference_to_commit);

        if commit_id.is_empty() {
            return Ok(());
        }

        let RGitObjectTypes::Commit(object) = repository
            .get_object(&commit_id)
            .expect("Branch aponta para referência inválida") else {
                panic!("Branch aponta para referência que não é um commit");
            };

        instanciate_commit(object, &mut repository);
    }

    Ok(())
}


fn prompt_uncommited_changes(repo: &Repository) -> Result<(), String> {
    let uncommited_files = get_uncommited_files(repo);

    if !uncommited_files.is_empty() {
        let file_list = uncommited_files.iter()
            .map(|f| format!("- {}\n", f.to_str().unwrap()))
            .collect::<String>();
        
        println!("As mudanças nos seguintes arquivos serão perdidas:");
        println!();
        println!("{}", file_list);
        println!("Deseja continuar? (y/n): ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            return Err("Operação de checkout abortada pelo usuário.".to_string());
        }
    }

    Ok(())
}

/// Instancia o commit ou a tree na worktree do repositório
/// 
/// Essa função deve dar pânico se algum erro ocorrer, pois isso indica que o repositório está corrompido.
pub fn instanciate_commit(object: CommitObject, repository: &mut Repository) {
    let final_tree_object = {
        // Assumimos apenas uma tree
        let tree = object.tree;

        let tree_object = repository.get_object(&tree).expect("Objeto da tree não foi encontrado (estado corrompido)");

        match tree_object {
            RGitObjectTypes::Tree(tree_object) => {
                tree::instanciate_tree_files(repository, &tree_object);
                tree_object
            }
            _ => {
                panic!("Tree do commit não é uma arvore.");
            }
        }
    };

    let tree_files = tree::get_tree_as_map(repository, &final_tree_object);
    let relative_file_paths: Vec<String> = tree_files.keys().cloned().collect();

    let new_staging_area = staging::StagingArea {
        entries: relative_file_paths.iter().map(|path| {
            let full_path = repository.worktree.join(path);
            let object_hash = tree_files.get(path).unwrap().clone();
            let relative_path = full_path.strip_prefix(&repository.worktree).unwrap();

            StagingEntry {
                last_content_change: object.timestamp,
                mode_type: 0o100644, // arquivo normal
                object_hash: object_hash.to_string(),
                path: relative_path.to_path_buf(),
            }
        }).collect()
    };

    staging::rewrite_index(repository, &new_staging_area);
}

