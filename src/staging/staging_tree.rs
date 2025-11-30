use std::{collections::HashMap, path::PathBuf};

use crate::{Repository, staging::StagingArea};

pub enum StagingTree {
    Blob(String),
    Fork(HashMap<String, Box<StagingTree>>)
}

impl StagingTree {
    pub fn insert(&mut self, blob_id: String, path: PathBuf) {
        let mut components = path.components();

        if let Some(root) = components.next() {
            let next = components.next();
            let root_str = root.as_os_str().to_str().unwrap().to_string();

            match next {
                None => {
                    match self {
                        StagingTree::Blob(_) => {}
                        StagingTree::Fork(children) => {
                            let child_name = root.as_os_str().to_str().unwrap().to_string();
                            children.insert(child_name, Box::new(StagingTree::Blob(blob_id)));
                        }
                    }
                },
                Some(_) => {
                    match self {
                        StagingTree::Blob(_) => {},
                        StagingTree::Fork(children) => {
                            let mut new_tree = StagingTree::Fork(HashMap::new());
                            let path_without_root = path.strip_prefix(root.as_os_str()).unwrap().to_path_buf();
                            new_tree.insert(blob_id, path_without_root);

                            children.insert(root_str, Box::new(new_tree));
                        }
                    }
                }
            }

            return;
        }

        unreachable!();
    }
}

pub fn instantiate_staging_tree_from_index(repo: &mut Repository) -> StagingTree {
    let staging_area = StagingArea::new(repo);
    let mut staging_tree = StagingTree::Fork(HashMap::new());

    for entry in staging_area.entries {
        staging_tree.insert(entry.object_hash, entry.path);
    }

    staging_tree
}