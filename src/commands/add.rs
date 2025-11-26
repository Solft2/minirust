use std::path::PathBuf;
use crate::staging::StagingArea;
use crate::Repository;

/// Adiciona um arquivo na área de staging
pub fn cmd_add(file_path: &str) {
    let file_path = PathBuf::from(file_path);

    let current = std::env::current_dir().expect("Deveria acessar o repositório atual");
    let mut repo = Repository::new(&current);

    if !repo.minigitdir.exists() {
        println!("Este diretório não é um repositório Minigit válido. Crie um com o comando 'minigit init'.");
        return;
    }

    let mut staging_area = StagingArea::new(&mut repo);
    staging_area.update_or_create_entry(&file_path);
    
    staging_area.save();
}