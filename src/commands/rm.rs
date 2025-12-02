use std::fs;

use crate::{checks::ensure_no_detached_head, commands::add::get_paths_relative_to_repository, staging::{StagingArea}, utils::find_current_repo};

pub fn cmd_rm(files: Vec<String>) {
    match cmd_rm_result(files) {
        Ok(_) => {}
        Err(e) => eprintln!("Erro ao remover arquivos do índice: {}", e),
    }
}

fn cmd_rm_result(files: Vec<String>) -> Result<(), String> {
    let repo = find_current_repo().ok_or("Não é um repositório minigit")?;

    ensure_no_detached_head(&repo)?;

    let repository_paths = get_paths_relative_to_repository(&repo, &files);
    let mut staging_area = StagingArea::new(&repo);
    for path in repository_paths {
        staging_area.remove_entry_with_path(&path);
    }

    let index_bytes = staging_area.serialize();
    fs::write(repo.index_path, index_bytes).unwrap();

    Ok(())
}