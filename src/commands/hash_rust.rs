use std::fs;
use std::path::PathBuf;
use crate::objects::{BlobObject, RGitObject};
use crate::utils::find_current_repo;

pub fn cmd_hash_object(path: &str, write:bool){
    match hash_object_command(path,write) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub fn hash_object_command(path: &str, write: bool) -> Result<(), String> {
    let mut repo = find_current_repo().ok_or("Não é um repositório minigit")?;
    let file_path = PathBuf::from(path);

    if !file_path.exists() || !file_path.is_file() {
        return Err("Caminho do arquivo inválido".to_string());
    }

    let bytes = fs::read(file_path).map_err(|_| "Não foi possível ler o arquivo".to_string())?;
    let blob =  BlobObject { content: bytes } ;

    println!("Hash SHA-1 do arquivo: {}", blob.hash());

    if write {
        repo.create_object(&blob);
        println!("Objeto blob escrito no repositório.");
    }

    Ok(())
}