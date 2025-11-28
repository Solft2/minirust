use std::{fs::{self, File}, io::{BufReader, Read, Write}, path::PathBuf};

pub fn create_file(path: &PathBuf, content: &Vec<u8>) {
    let mut file = File::create(path).expect("Deveria criar o arquivo.");
    file.write(&content).expect("Deveria escrever no arquivo.");
}

pub fn create_dir(path: &PathBuf) {
    fs::create_dir(path).expect("Deveria criar o diretório");
}

/// Lê o conteúdo de um arquivo e retorna como vetor de bytes
/// ## Argumentos
/// - `path` - Caminho do arquivo a ser lido
pub fn read_bytes_from_file(path: &PathBuf) -> Vec<u8> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut content = Vec::new();
    reader.read_to_end(&mut content).unwrap();
    
    content
}

/// Escreve um vetor de bytes em um arquivo
/// ## Argumentos
/// - `path` - Caminho do arquivo a ser escrito
/// - `content` - Conteúdo a ser escrito
pub fn write_bytes_to_file(path: &PathBuf, content: &Vec<u8>) {
    let mut file = File::create(path).unwrap();
    file.write_all(content).unwrap();
}

/// Lê o conteúdo de um arquivo e retorna como String
/// Assume que o conteúdo do arquivo é UTF-8
/// 
/// ## Argumentos
/// - `path` - Caminho do arquivo a ser lido
pub fn read_string_from_file(path: &PathBuf) -> String {
    let content = read_bytes_from_file(path);
    
    String::from_utf8(content).unwrap()
}

/// Escreve uma String em um arquivo
/// Assume que o conteúdo é UTF-8
/// ## Argumentos
/// - `path` - Caminho do arquivo a ser escrito
/// - `content` - Conteúdo a ser escrito
pub fn write_string_to_file(path: &PathBuf, content: &String) {
    write_bytes_to_file(path, &content.as_bytes().to_vec());
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