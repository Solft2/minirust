use std::{fs::{self, File}, io::Write, path::PathBuf};

pub fn create_file(path: &PathBuf, content: &Vec<u8>) {
    let mut file = File::create(path).expect("Deveria criar o arquivo.");
    file.write(&content).expect("Deveria escrever no arquivo.");
}

pub fn create_dir(path: &PathBuf) {
    fs::create_dir(path).expect("Deveria criar o diretório");
}