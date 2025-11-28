use core::panic;
use std::path::PathBuf;

use crate::{Repository, objects::{RGitObjectTypes, TreeObject}, utils::find_current_repo};

pub fn cmd_ls_tree(tree_id: String) {
    match cmd_ls_tree_result(tree_id) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_ls_tree_result(tree_id: String) -> Result<(), String> {
    let repo = find_current_repo().ok_or("Não está dentro de um repositório")?;
    
    let object = repo.get_object(&tree_id)
        .ok_or("Objeto não encontrado no repositório")?;

    println!("mode object_id\tpath");
    match object {
        RGitObjectTypes::Tree(tree) => {
            print_tree(&tree, &PathBuf::new(), &repo);
        }
        RGitObjectTypes::Commit(commit) => {
            let tree = repo.get_object(&commit.tree).unwrap();

            match tree {
                RGitObjectTypes::Tree(tree) => print_tree(&tree, &PathBuf::new(), &repo),
                _ => panic!("A tree referenciada pelo commit não é válida."),
            }
        }
        _ => {
            return Err("O objeto fornecido não é uma árvore".to_string());
        }
    }
    Ok(())
}

fn print_tree(tree: &TreeObject, prefix: &PathBuf, repo: &Repository) {
    for child in &tree.children {
        let child_object = repo.get_object(&child.object_id).unwrap();

        match child_object {
            RGitObjectTypes::Blob(_) => {
                println!("{} {} {}", child.mode, child.object_id, prefix.join(&child.name).display());
            },
            RGitObjectTypes::Tree(child_tree) => {
                let mut new_prefix = prefix.clone();
                new_prefix.push(&child.name);

                print_tree(&child_tree, &new_prefix, repo);
            },
            _ => {}
        }
    }
}