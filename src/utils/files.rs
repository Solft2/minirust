use std::{fs::{self, File}, io::{BufReader, Read, Write}, path::PathBuf};

pub fn create_file(path: &PathBuf, content: &Vec<u8>) {
    let mut file = File::create(path).expect("Deveria criar o arquivo.");
    file.write(&content).expect("Deveria escrever no arquivo.");
}

pub fn create_dir(path: &PathBuf) {
    fs::create_dir(path).expect("Deveria criar o diretório");
}

/// Lê o conteúdo de um arquivo e retorna como String
/// Assume que o conteúdo do arquivo é UTF-8
/// 
/// ## Argumentos
/// - `path` - Caminho do arquivo a ser lido
pub fn read_string_from_file(path: &PathBuf) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = Vec::new();
    reader.read_to_end(&mut content).unwrap();
    
    String::from_utf8(content).unwrap()
}

pub fn write_string_to_file(path: &PathBuf, content: &String) {
    println!("{:?}", path);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}