use std::{path::PathBuf};

use crate::{checks::ensure_no_detached_head, utils::find_current_repo};

/// Adiciona um arquivo na área de staging
pub fn cmd_add(files: Vec<String>) {
    match cmd_add_result(files) {
        Ok(_) => {}
        Err(e) => {
            println!("Erro ao adicionar arquivo: {}", e);
        }
    }
}

fn cmd_add_result(files: Vec<String>) -> Result<(), String> {
    let curent_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    let mut repo = find_current_repo().ok_or("Não é um repositório minigit")?;

    ensure_no_detached_head(&repo)?;

    let paths = files.into_iter().map(|f| {
        let blob_path = PathBuf::from(&f);
        curent_dir
            .join(blob_path)
            .strip_prefix(&repo.worktree)
            .unwrap()
            .to_path_buf()
    })
    .filter(|path| {
        let absolute_path = repo.worktree.join(path);
        if !absolute_path.is_file() {
            println!("Aviso: {:?} não é um arquivo regular e será ignorado.", path);
        }
        absolute_path.is_file()
    }).collect();
    
    repo.add_files(paths);

    Ok(())
}