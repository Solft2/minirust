use std::fs;
use crate::utils::{find_current_repo, is_valid_sha1};

pub fn cmd_cat_file(hash: &String){
    match cat_file_command(hash) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cat_file_command(hash: &String) -> Result<(), String> {
    let repo = find_current_repo().ok_or("Não é um repositório minigit")?;

    if !is_valid_sha1(hash) {
        return Err("Hash SHA-1 inválido".to_string());
    }

    let (dir, file_name) = hash.split_at(2);
    let path = repo.minigitdir.join("objects").join(dir).join(file_name);

    if !path.exists() {
        return Err("Objeto não encontrado".to_string());
    }

    let raw_object = fs::read(&path).unwrap();
    let space_pos = raw_object.iter().position(|b| *b == b' ').unwrap();
    let (object_type, remainder) = raw_object.split_at(space_pos);

    let null_pos = remainder.iter().position(|b| *b == b'\0').unwrap();
    let (_size_bytes, object_content) = remainder.split_at(null_pos + 1);

    let object_type = String::from_utf8_lossy(object_type).to_string();

    println!("Tipo de objeto: {}", object_type);
    println!("Tamanho do conteúdo: {} bytes", object_content.len());
    println!("Conteúdo: \n{}", String::from_utf8_lossy(object_content));

    Ok(())

}

