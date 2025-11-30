use std::{collections::HashMap, path::PathBuf};

use crate::{utils, Repository, objects::{RGitObject, RGitObjectTypes}, staging::StagingTree};

pub struct TreeObject {
    pub children: Vec<TreeObjectChild>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeObjectChild {
    pub mode: String,
    pub object_id: String,
    pub name: String
}

impl RGitObject for TreeObject {
    fn serialize(&self) -> Vec<u8> {
        let mut result = String::new();

        for child in &self.children {
            let helper = format!("{} {}\0{}\n", &child.mode, &child.name, &child.object_id);
            result.push_str(&helper);
        }

        result.as_bytes().to_vec()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        self.children = Self::children_from_bytes(object_bytes);
    }

    fn object_type(&self) -> &'static str {
        "tree"
    }
}

impl TreeObject {
    pub fn new(object_bytes: Vec<u8>) -> Self {
        TreeObject { children: Self::children_from_bytes(object_bytes) }
    }

    fn children_from_bytes(object_bytes: Vec<u8>) -> Vec<TreeObjectChild> {
        let mut result: Vec<TreeObjectChild> = Vec::new();
        let mut object_str = str::from_utf8(&object_bytes).expect("O objeto deve ser uma string UTF-8 válida");

        while let Some(new_line) = object_str.find('\n') {
            let record = &object_str[..new_line];
            result.push(Self::parse_child(record));
            object_str = &object_str[new_line+1..];
        }

        result.sort();

        result
    }

    fn parse_child(record: &str) -> TreeObjectChild {
        let space = record.find(' ');
        let null = record.find('\0');

        if space.is_none() || null.is_none() {
            panic!("Objeto árvore mal formatado");
        }

        let space = space.unwrap();
        let null = null.unwrap();

        TreeObjectChild { 
            mode: record[..space].to_string(), 
            name: record[space+1..null].to_string(),
            object_id: record[null+1..].to_string() 
        }
    }
}


/// Transforma a árvore do commit em um HashMap de caminho relativo ao repositório -> hash do objeto
pub fn get_tree_as_map(repo: &Repository, tree: &TreeObject) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let curr_dir = PathBuf::new();

    get_tree_as_map_recursive(repo, tree, curr_dir, &mut result);

    result
}

/// Função recursiva auxiliar para transformar a árvore do commit em um HashMap
/// 
/// Resultado final é armazenado em `result`
/// 
/// `curr_dir` é RELATIVO ao working directory do repositório
pub fn get_tree_as_map_recursive(repo: &Repository, tree: &TreeObject, curr_dir: PathBuf, result: &mut HashMap<String, String>) {
    for child in &tree.children {
        let object = repo.get_object(&child.object_id).unwrap();

        match object {
            RGitObjectTypes::Blob(blob_obj) => {
                let file_path = curr_dir.clone().join(&child.name);
                let file_path_str = file_path.to_str().unwrap().to_string();

                result.insert(file_path_str, blob_obj.hash());
            }
            RGitObjectTypes::Tree(tree_obj) => {
                let new_dir = curr_dir.clone().join(&child.name);

                get_tree_as_map_recursive(repo, &tree_obj, new_dir, result);
            }
            _ => panic!("Objeto inválido na árvore do commit"),
        }
    }
}

pub fn create_tree_object_from_staging_tree(staging_tree: &StagingTree, repo: &mut Repository) -> String {
    let mut object: TreeObject = TreeObject {
        children: Vec::new()
    };

    match staging_tree {
        StagingTree::Blob(blob_id) => {
            return blob_id.clone();
        },
        StagingTree::Fork(children) => {
            for (name, child) in children {
                let child_id = create_tree_object_from_staging_tree(child, repo);

                let tree_child = TreeObjectChild {
                    mode: "100644".to_string(),
                    object_id: child_id,
                    name: name.clone(),
                };

                object.children.push(tree_child);
            }
        }
    }

    repo.create_object(&object);

    object.hash()
}

pub fn instanciate_tree_files(repository: &mut Repository, tree: &TreeObject) {
    instanciate_subtree_files(repository, tree, &repository.worktree.clone());
}

fn instanciate_subtree_files(repository: &mut Repository, tree: &TreeObject, current_dir: &PathBuf) {
    for child in &tree.children {
        let object = repository
            .get_object(&child.object_id)
            .expect("Objeto da tree deveria existir");
        
        let path = current_dir.join(child.name.clone());

        match object {
            RGitObjectTypes::Blob(blob) => {
                utils::create_file(&path, &blob.content);
            },
            RGitObjectTypes::Tree(tree) => {
                utils::create_dir(&path);

                instanciate_subtree_files(repository, &tree, &path);
            }
            _ => {
                panic!("Objeto não é blob ou tree.");
            }
        }
    }
}