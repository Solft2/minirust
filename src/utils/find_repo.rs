use std::path::PathBuf;

use crate::Repository;

pub fn find_repo(current_path: &PathBuf) -> Option<Repository> {
    let mut workdir = current_path.clone();
    while !has_repository(&workdir) {
        if !workdir.pop() {
            break;
        }
    }

    if let Some(_parent) = workdir.parent() {
        Some(Repository::new(&workdir))
    } else {
        None
    }
}

fn has_repository(path: &PathBuf) -> bool {
    path.join(Repository::MINIGITDIR).exists()
}