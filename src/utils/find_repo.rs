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

pub fn find_current_repo() -> Option<Repository> {
    let current_path = std::env::current_dir().unwrap();
    find_repo(&current_path)
}

fn has_repository(path: &PathBuf) -> bool {
    path.join(Repository::MINIGITDIR).exists()
}