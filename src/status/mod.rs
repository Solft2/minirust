use std::{collections::HashMap, fs, path::PathBuf};

use walkdir::WalkDir;

use crate::{Repository, config::RGitIgnore, objects::{CommitObject, RGitObjectTypes}, staging::{StagingArea, StagingEntry}};

/// Retorna uma lista de arquivos não adicionados (modificados ou novos) no repositório
pub fn non_staged_files(repo: &Repository) -> Vec<PathBuf> {
    let ignore = RGitIgnore::new(repo);
    let staging_area = StagingArea::new(repo);
    let staging_area_entries_map = staging_area.get_entries_as_map();

    let all_files = WalkDir::new(&repo.worktree)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| get_relative_path(repo, &e.path().to_path_buf()))
        .filter(|path| is_not_ignored(&ignore, path))
        .collect::<Vec<PathBuf>>();

    all_files.iter()
        .filter(|path| is_non_staged(repo, &staging_area_entries_map, *path))
        .cloned()
        .collect()
}


pub fn get_uncommited_files(repo: &Repository) -> Vec<PathBuf> {
    let ignore = RGitIgnore::new(repo);
    let staging_area = StagingArea::new(repo);
    let staging_area_entries_map = staging_area.get_entries_as_map();
    let last_commit_hash = repo.resolve_head();

    let all_files = WalkDir::new(&repo.worktree)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| get_relative_path(repo, &e.path().to_path_buf()))
        .filter(|path| is_not_ignored(&ignore, path))
        .collect::<Vec<PathBuf>>();

    if last_commit_hash.is_empty() {
        return all_files;
    }

    let RGitObjectTypes::Commit(commit_object) = repo.get_object(&last_commit_hash).unwrap()
        else { panic!("Não é commit") };

    all_files.iter()
        .filter(|path| is_uncommited(repo, &staging_area_entries_map, *path, &commit_object))
        .cloned()
        .collect()
}

fn get_relative_path(repo: &Repository, full_path: &PathBuf) -> PathBuf {
    full_path.strip_prefix(&repo.worktree).unwrap().to_path_buf()
}

fn is_not_ignored(ignore: &RGitIgnore, relative_path: &PathBuf) -> bool {
    !ignore.check_ignore(relative_path)
}

fn is_non_staged(repo: &Repository, staging_area_entries_map: &HashMap<PathBuf, StagingEntry>,  relative_path: &PathBuf) -> bool {
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
                .as_nanos();

            last_staged != last_modified
        },
    }
}

fn is_uncommited(
    repo: &Repository, 
    staging_area_entries_map: &HashMap<PathBuf, StagingEntry>,  
    relative_path: &PathBuf,
    last_commit: &CommitObject
) -> bool {
    let entry_opt: Option<&StagingEntry> = staging_area_entries_map.get(relative_path);

    match entry_opt {
        None => true,
        Some(entry) => {
            let last_staged = entry.last_content_change.clone();

            is_non_staged(repo, staging_area_entries_map, relative_path) ||
            last_staged != last_commit.timestamp
        },
    }
}