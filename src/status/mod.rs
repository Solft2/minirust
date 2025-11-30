use std::{fs, path::PathBuf};

use walkdir::WalkDir;

use crate::{Repository, config::RGitIgnore, staging::StagingArea};

pub fn non_staged_files(repo: &Repository) -> Vec<PathBuf> {
    let ignore = RGitIgnore::new(repo);
    let staging_area = StagingArea::new(repo);
    let staging_area_entries_map = staging_area.get_entries_as_map();

    let get_relative_path = |entry: walkdir::DirEntry| {
        entry.path().strip_prefix(&repo.worktree).unwrap().to_path_buf()
    };
    let is_not_ignored = |relative_path: &PathBuf| {
        !ignore.check_ignore(relative_path)
    };
    let is_not_staged = |relative_path: &PathBuf| {
        let entry_opt = staging_area_entries_map.get(relative_path);

        match entry_opt {
            None => true,
            Some(entry) => {
                let last_staged = entry.last_content_change.clone();
                let full_path = repo.worktree.join(relative_path);
                let last_modified = fs::metadata(&full_path).unwrap().modified().unwrap();
                let last_modified = last_modified
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                last_staged != last_modified
            },
        }
    };

    let all_files = WalkDir::new(&repo.worktree)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(get_relative_path)
        .filter(is_not_ignored)
        .collect::<Vec<PathBuf>>();

    all_files.iter()
        .filter(|path| is_not_staged(*path))
        .cloned()
        .collect()
}