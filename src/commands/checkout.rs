use crate::{Repository, utils::find_repo};


pub fn cmd_checkout(commit_id: &String) {
    let current_dir = std::env::current_dir().expect("Deveria acessar o diretório atual");
    let repository = find_repo(&current_dir);

    match repository {
        None => {
            println!("Repositório minigit não encontrado");
            return;
        }
        Some(repository) =>  {
            let obj = repository.get_object(commit_id);
        }
    }
}