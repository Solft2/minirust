use std::collections::HashMap;

use crate::{Repository, objects::{CommitObject, RGitObject, TreeObject, TreeObjectChild}, staging::{StagingArea, StagingTree}, utils::{find_repo}};

pub fn cmd_commit(message: String) {
    match cmd_commit_result(message) {
        Ok(hash) => {
            println!("Commit criado com o hash {}", hash);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_commit_result(message: String) -> Result<String, String> {
    let current_path = &std::env::current_dir().unwrap();

    let mut repo = find_repo(current_path)
        .ok_or("Diretório não está dentro um repositório minigit")?;

    let staging_tree = instantiate_tree_from_index(&mut repo);

    let tree_id = create_tree_object(&staging_tree, &mut repo);
    let author = "Ian";
    let head = repo.resolve_head();
    let parent: Vec<String> = if head.is_empty() {
        Vec::new()
    } else {
        vec![head.clone()]
    };
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    let commit = CommitObject {
        tree: tree_id,
        author: author.to_string(),
        message: message,
        timestamp: now,
        parent: parent,
    };

    repo.create_object(&commit);
    repo.update_head(&commit.hash());

    Ok(commit.hash())
}

fn instantiate_tree_from_index(repo: &mut Repository) -> StagingTree {
    let staging_area = StagingArea::new(repo);
    let mut staging_tree = StagingTree::Fork(HashMap::new());

    for entry in staging_area.entries {
        staging_tree.insert(entry.object_hash, entry.path);
    }

    staging_tree
}

fn create_tree_object(staging_tree: &StagingTree, repo: &mut Repository) -> String {
    let mut object = TreeObject {
        children: Vec::new()
    };

    match staging_tree {
        StagingTree::Blob(blob_id) => {
            return blob_id.clone();
        },
        StagingTree::Fork(children) => {
            for (name, child) in children {
                let child_id = create_tree_object(child, repo);

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