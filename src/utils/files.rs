use std::{fs::{self, File}, io::{Write}, path::PathBuf};

pub fn create_file(path: &PathBuf, content: &Vec<u8>) {
    let mut file = File::create(path).expect("Deveria criar o arquivo.");
    file.write(&content).expect("Deveria escrever no arquivo.");
}

pub fn create_dir(path: &PathBuf) {
    fs::create_dir(path).expect("Deveria criar o diretório");
}

pub fn get_current_dir() -> PathBuf {
    std::env::current_dir().expect("Deveria obter o diretório atual")
}

/// Lê um valor do conteúdo de um arquivo no formato `<chave> <valor>`, retornando o valor lido e o restante do conteúdo
/// 
/// Retorna a chave, o valor e o restante do conteúdo.
/// O valor pode conter múltiplas linhas, desde que cada linha subsequente comece com um espaço
pub fn read_value(content: &str) -> (String, String, &str) {
    let parts: Vec<&str> = content.splitn(2, ' ').collect();
    let key = parts[0].to_string();
    let mut remainder = parts[1];

    let new_line = remainder.find('\n').unwrap_or(remainder.len());
    let mut value = remainder[..new_line].to_string();
    remainder = &remainder[new_line+1..];

    while remainder.starts_with(' ') {
        let new_line = remainder.find('\n').unwrap_or(remainder.len());
        value.push('\n');
        value.push_str(&remainder[..new_line]);
        remainder = &remainder[new_line+1..];
    }

    (key, value, remainder)
}