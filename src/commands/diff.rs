use std::fs::read_to_string;
use crate::objects::{get_tree_as_map, RGitObjectTypes};
use crate::status::{non_staged_files};
use crate::utils::find_current_repo;

pub fn cmd_diff() {
    let repository = find_current_repo().unwrap();
    let non_staged = non_staged_files(&repository);

    let head_commit = repository.resolve_head();
    let RGitObjectTypes::Commit(commit) = repository.get_object(&head_commit).unwrap()
    else { panic!("Objeto do head commit não é um commit."); };
    let RGitObjectTypes::Tree(tree_object) = repository.get_object(&commit.tree).unwrap()
    else { panic!("Objeto da tree do head commit não é um commit."); };

    let tree_files = get_tree_as_map(&repository, &tree_object);

    for path in non_staged {
        let path_string = path.to_str().unwrap();
        let current_content = match tree_files.get(path_string) {
            Some(blob_hash_string) => {
                let RGitObjectTypes::Blob(blob) = repository.get_object(blob_hash_string).unwrap()
                else { panic!("O objeto não é um blob!"); };

                String::from_utf8(blob.content).unwrap()
            }
            None => String::new(),
        };

        let new_content = read_to_string(repository.worktree.join(path_string)).unwrap_or_default();
        println!("{}:", path_string);
        find_content_differences(&current_content, &new_content);
    }
}


fn find_content_differences(old_text: &str, new_text: &str) {
    let old: Vec<&str> = old_text.lines().collect();
    let new: Vec<&str> = new_text.lines().collect();
    let bigger_len: usize;

    if old.len() > new.len() {
        bigger_len = old.len();
    } else {
        bigger_len = new.len();
    }

    for i in 0..bigger_len {
        let old_line = old.get(i).unwrap_or(&"");
        let new_line = new.get(i).unwrap_or(&"");

        if old_line != new_line {
            println!("  Linha {}:", i + 1);
            if old_line.len() != 0 {
                println!("      - {}", old_line);
            }
            if new_line.len() != 0{
                println!("      + {}", new_line);
            }
        }
    }
}