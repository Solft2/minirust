use crate::{Repository, staging, objects::{RGitObjectTypes, tree}, utils::{find_current_repo, is_valid_sha1, resolve_head_or_branch_name}};

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

    let is_commit_id = is_valid_sha1(&reference_to_commit);

    if is_commit_id {
        repository.clear_worktree();

        let object = repository
            .get_object(&reference_to_commit)
            .ok_or("Não é um commit reconhecido pelo minigit")?;

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

        let object = repository
            .get_object(&commit_id)
            .expect("Branch aponta para referência inválida");

        instanciate_commit(object, &mut repository);
    }

    Ok(())
}


/// Instancia o commit ou a tree na worktree do repositório
/// 
/// Essa função deve dar pânico se algum erro ocorrer, pois isso indica que o repositório está corrompido.
pub fn instanciate_commit(object: RGitObjectTypes, repository: &mut Repository) {
    let final_tree_object = match object {
        RGitObjectTypes::Commit(commit) => {
            // Assumimos apenas uma tree
            let tree = commit.tree;

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
        }
        RGitObjectTypes::Tree(tree_object) => {
            tree::instanciate_tree_files(repository, &tree_object);
            tree_object
        }
        _ => {
            panic!("Objeto não é um commit ou uma tree");
        }
    };

    let tree_files = tree::get_tree_as_map(repository, &final_tree_object);
    let relative_file_paths: Vec<String> = tree_files.keys().cloned().collect();
    let new_staging_area = staging::StagingArea {
        entries: relative_file_paths.iter().map(|path| {
            let full_path = repository.worktree.join(path);
            let object_hash = tree_files.get(path).unwrap().clone();

            staging::StagingEntry::from((&full_path, &object_hash, &repository.worktree))
        }).collect()
    };

    staging::rewrite_index(repository, &new_staging_area);
}

