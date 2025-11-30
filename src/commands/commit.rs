use std::collections::HashMap;

use crate::{Repository, objects::{CommitObject, RGitObject, create_tree_object_from_staging_tree}, staging::{StagingArea, StagingTree}, utils::find_current_repo};

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
    let mut repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    if repo.head_detached() {
        return Err("Não é possível criar um commit com o HEAD destacado".to_string());
    }

    let staging_tree = instantiate_tree_from_index(&mut repo);

    let tree_id = create_tree_object_from_staging_tree(&staging_tree, &mut repo);

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

    repo.create_object(&commit);
    repo.update_curr_branch(&commit.hash());

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
