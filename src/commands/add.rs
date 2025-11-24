use std::path::PathBuf;
use crate::staging::StagingArea;
use crate::Repository;

pub fn cmd_add() {
    let file_path = PathBuf::from("teste.txt");

    let current = std::env::current_dir().expect("Não foi possível acessar o diretório");
    let mut repo = Repository::new(&current);
    repo.get_rgitdir().expect("Não é um repositório RGit válido. Crie um com 'rgit init'.");

    let mut staging_area = StagingArea::new(&mut repo);
    staging_area.update_or_create_entry(&file_path);
    
    staging_area.save();
}