use std::{collections::HashMap};

use crate::{Repository, objects::{RGitObject, RGitObjectTypes, create_tree_object_from_staging_tree, get_tree_as_map}, staging::instantiate_staging_tree_from_index, utils::files};

#[derive(Debug, Clone)]
pub struct CommitObject {
    pub tree: String,
    pub author: String,
    pub message: String,
    pub timestamp: u128,
    pub parent: Vec<String>
}

impl CommitObject {
    pub fn new(content_bytes: Vec<u8>) -> Self {
        let content_str = String::from_utf8(content_bytes).expect("Commit deve ser um arquivo UTF8 válido");
        
        let (_, tree, remainder) = files::read_value(&content_str);
        let (_, author, remainder) = files::read_value(remainder);
        let (_, message, remainder) = files::read_value(remainder);
        let (_, timestamp_str, mut remainder) = files::read_value(remainder);

        let timestamp: u128 = timestamp_str.parse().expect("Timestamp deve ser um número válido");
        let mut parent: Vec<String> = Vec::new();

        while !remainder.is_empty() {
            let (_, parent_commit, new_remainder) = files::read_value(remainder);
            parent.push(parent_commit);
            remainder = new_remainder;
        }

        Self { tree, author, message, timestamp, parent }
    }
}

impl RGitObject for CommitObject {
    /// Ordem precisa ser a mesma da entrada (tree, author, message, timestamp, parent...)
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        let message = self.message.replace("\n", "\n ");
        let parent = self.parent.iter()
            .map(|p| p.replace("\n", "\n "))
            .collect::<Vec<String>>();

        result.extend_from_slice(format!("tree {}\n", self.tree).as_bytes());
        result.extend_from_slice(format!("author {}\n", self.author).as_bytes());
        result.extend_from_slice(format!("message {}\n", message).as_bytes());
        result.extend_from_slice(format!("timestamp {}\n", self.timestamp).as_bytes());
        for parent in &parent {
            result.extend_from_slice(format!("parent {}\n", parent).as_bytes());
        }

        result
    }

    #[allow(unused_variables)]
    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        panic!("Não implementado");
    }

    fn object_type(&self) -> &'static str {
        "commit"
    }
}

/// Cria um objeto de commit a partir do índice atual do repositório.
/// 
/// Retorna o hash do commit criado.
pub fn create_commit_object_from_index(repo: &mut Repository, message: String) -> String {
    let staging_tree = instantiate_staging_tree_from_index(repo);

    let tree_id = create_tree_object_from_staging_tree(&staging_tree, repo);

    let author_name = repo.config.get_username();
    let author_email = repo.config.get_email();
    let author = format!("{} <{}>", author_name, author_email);
    
    let head = repo.resolve_head();
    let parent: Vec<String> = if head.is_empty() {
        Vec::new()
    } else {
        vec![head.clone()]
    };
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();

    let commit = CommitObject {
        tree: tree_id,
        author: author.to_string(),
        message: message,
        timestamp: now,
        parent: parent,
    };

    return repo.create_object(&commit);
}

/// Transforma o commit em um HashMap de caminho de arquivo para blob hash
pub fn get_commit_tree_as_map(repo: &Repository, commit: &CommitObject) -> HashMap<String, String>{
    let commit_tree = repo.get_object(&commit.tree).unwrap();

    match commit_tree {
        RGitObjectTypes::Tree(tree_obj) => {
            get_tree_as_map(repo, &tree_obj)
        }
        _ => panic!("Objeto de árvore esperado para o commit"),
    }
}