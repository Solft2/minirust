use std::path::PathBuf;
use fs_extra::dir::{copy, CopyOptions};
use crate::commands::checkout::{instanciate_commit};
use crate::Repository;
use crate::objects::RGitObjectTypes;
use crate::utils::{find_repo};

pub fn cmd_clone(repository_path: &str, destination_path: &str) {
    match execute_clone(repository_path, destination_path) {
        Ok(..) => {
            println!("Repositório clonado com sucesso!")
        },
        Err(err) => {
            println!("Erro: {}.", err);
        }
    }
}

pub fn execute_clone(repository_path: &str, destination_path: &str) -> Result<(), String> {
    let source = PathBuf::from(repository_path);
    let destination = PathBuf::from(destination_path);

    let source_repository = find_repo(&source)
        .ok_or("Não há um repositório minigit nesse caminho de origem. Use o comando init para inicializar um novo repositório.")?;
    if destination.exists() {
        return Err(String::from("Já existe um diretório com esse nome nesse mesmo local."));
    }

    std::fs::create_dir_all(&destination).map_err(|e| e.to_string())?;

    let mut new_repository = Repository::new(destination.as_path());
    copy_minigit_dir(&source_repository, &new_repository);

    let head_commit = new_repository.resolve_head();

    let RGitObjectTypes::Commit(object) = new_repository
        .get_object(&head_commit)
        .ok_or("Erro ao copiar diretório. Head commit do repositório não encontrado.")?
        else { panic!("Objeto do head commit não é um commit."); };

    instanciate_commit(object, &mut new_repository);
    Ok(())
}

fn copy_minigit_dir(source: &Repository, new_repository: &Repository) {
    let mut options = CopyOptions::new();
    options.copy_inside = true;
    options.overwrite = true;

    match copy(&source.minigitdir, &new_repository.minigitdir, &options) {
        Ok(_) => println!("Pasta minigit copiada com sucesso"),
        Err(e) => println!("Erro ao copiar a pasta minigit: {}", e),
    }
}