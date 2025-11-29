use std::{path::PathBuf};

use crate::{Repository, objects::{RGitObjectTypes, TreeObject}, utils::{create_dir, create_file, find_repo}};


pub fn cmd_checkout(commit_id: &String) {
    match execute_checkout(commit_id) {
        Ok(..) => {
            println!("Indo para o commit {}", commit_id)
        },
        Err(err) => {
            println!("Erro: {}.", err);
        }
    }
}

fn execute_checkout(commit_id: &String) -> Result<(), String> {
    let current_dir = std::env::current_dir()
        .expect("Deveria conseguir ler o diretório atual");

    let mut repository = find_repo(&current_dir)
        .ok_or("Não é um repositório minigit")?;

    let object = repository
        .get_object(commit_id)
        .ok_or("Não existe um objeto com este ID")?;

    repository.clear_worktree();

    match object {
        RGitObjectTypes::Commit(commit) => {
            // Assumimos apenas uma tree
            let tree = commit.tree;

            let tree_object = repository.get_object(&tree).expect("Objeto da tree não foi encontrado (estado corrompido)");

            match tree_object {
                RGitObjectTypes::Tree(tree_object) => {
                    instanciate_tree(&mut repository, &tree_object);
                }
                _ => {
                    panic!("Tree do commit não é uma arvore.");
                }
            }
        }
        RGitObjectTypes::Tree(tree_object) => {
            instanciate_tree(&mut repository, &tree_object);
        }
        _ => {
            return Err(String::from("Objeto não é um commit ou uma tree"));
        }
    }



    Ok(())
}

fn instanciate_tree(repository: &mut Repository, tree: &TreeObject) {
    instanciate_subtree(repository, tree, &repository.worktree.clone());
}

fn instanciate_subtree(repository: &mut Repository, tree: &TreeObject, current_dir: &PathBuf) {
    for child in &tree.children {
        let object = repository
            .get_object(&child.object_id)
            .expect("Objeto da tree deveria existir");
        
        let path = current_dir.join(child.name.clone());
        let relative_path = path.strip_prefix(&repository.worktree).unwrap();

        match object {
            RGitObjectTypes::Blob(blob) => {
                create_file(&path, &blob.content);
                repository.add_files(vec![relative_path.to_path_buf()]);
            },
            RGitObjectTypes::Tree(tree) => {
                println!("Criando diretório {:?}", path);
                create_dir(&path);

                instanciate_subtree(repository, &tree, &path);
            }
            _ => {
                panic!("Objeto não é blob ou tree.");
            }
        }
    }
}