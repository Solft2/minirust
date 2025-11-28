use std::{collections::HashMap, path::PathBuf};

pub enum StagingTree {
    Blob(String),
    Fork(HashMap<String, Box<StagingTree>>)
}

impl StagingTree {
    pub fn insert(&mut self, blob_id: String, path: PathBuf) {
        if let Some(parent) = path.parent() {
            let parent_name = parent.to_str().unwrap();
            if parent_name == "" {
                match self {
                    StagingTree::Blob(_) => {}
                    StagingTree::Fork(children) => {
                        let child_name = path.to_str().unwrap().to_string();
                        children.insert(child_name, Box::new(StagingTree::Blob(blob_id)));
                    }
                }
            } else {
                match self {
                    StagingTree::Blob(_) => {},
                    StagingTree::Fork(children) => {
                        let mut new_tree = StagingTree::Fork(HashMap::new());
                        let path_without_parent = path.strip_prefix(parent).unwrap().to_path_buf();
                        new_tree.insert(blob_id, path_without_parent);

                        children.insert(parent_name.to_string(), Box::new(new_tree));
                    }
                }
            }

            return;
        }

        unreachable!();
    }
}