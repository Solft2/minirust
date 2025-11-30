use std::{path::PathBuf};

use crate::{Repository, checks::ensure_no_detached_head, config::RGitIgnore, utils::{find_current_repo, get_current_dir}};

/// Adiciona um arquivo na área de staging
pub fn cmd_add(files_to_add: Vec<String>) {
    if let Err(err) = cmd_add_result(files_to_add) {
        eprintln!("Erro ao adicionar arquivo(s): {}", err);
    }
}

fn cmd_add_result(files: Vec<String>) -> Result<(), String> {
    let mut repo = find_current_repo().ok_or("Não é um repositório minigit")?;

    ensure_no_detached_head(&repo)?;

    let repository_paths = get_paths_relative_to_repository(&repo, &files);
    let valid_paths = filter_invalid_paths(&repo, &repository_paths);
    let paths_to_add = filter_ignored_files(&repo, &valid_paths);
    
    repo.add_files(paths_to_add);

    Ok(())
}

/// Transforma paths relativos ao diretório atual em paths relativos à raiz do repositório
fn get_paths_relative_to_repository(repo: &Repository, files: &Vec<String>) -> Vec<PathBuf> {
    let current_dir = get_current_dir();

    files.into_iter().map(|f| {
        let blob_path = PathBuf::from(&f);
        current_dir
            .join(blob_path)
            .strip_prefix(&repo.worktree)
            .unwrap()
            .to_path_buf()
    }).collect()
}

/// Filtra caminhos que são diretórios ou inexistentes
fn filter_invalid_paths(repo: &Repository, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| {
            let absolute_path = repo.worktree.join(path);
            let is_valid = absolute_path.exists() && absolute_path.is_file();
            if !is_valid {
                println!("Aviso: {:?} não é um arquivo regular e será ignorado", path);
            }
            is_valid
        })
        .cloned()
        .collect()
}

/// Filtra caminhos que são ignorados pelo .gitignore
fn filter_ignored_files(repo: &Repository, paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let repo_ignore = RGitIgnore::new(repo);

    paths
        .iter()
        .filter(|path| {
            let is_ignored = repo_ignore.check_ignore(path);
            if is_ignored {
                println!("Aviso: {:?} está sendo ignorado pelo .gitignore e não será adicionado", path);
            }
            !is_ignored
        })
        .cloned()
        .collect()
}