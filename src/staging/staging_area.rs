use core::{panic};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::{collections::HashMap, path::PathBuf};
use crate::Repository;
use crate::objects::{BlobObject, RGitObject};

/// Uma entrada da staging area
pub struct StagingEntry {
    pub path: PathBuf,
    pub hash: String
}

impl StagingEntry {
    /// Converte a entrada de staging em um array de bytes para ser escrito no arquivo de índice
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut entry_str: String = String::new();
        entry_str.push_str(self.path.to_str().unwrap());
        entry_str.push(' ');
        entry_str.push_str(&self.hash);

        entry_str.as_bytes().to_vec()
    }
}

/// Representa a área de staging, onde os arquivos alterados vão antes de serem comitados.
/// 
/// A implementação é através de um arquivo que mantém um mapeamento de caminho -> hash do objeto.
/// 
/// Quando o usuário adicionar um arquivo em staging (comando add), devemos atualizar a entrada correspondente
pub struct StagingArea<'a> {
    pub entries: HashMap<String, StagingEntry>,
    pub repo: &'a mut Repository
}

impl<'a> StagingArea<'a> {
    const INDEX_FILE: &'static str = "index";

    /// Carrega a área de staging do arquivo `index`
    pub fn new(repo: &'a mut Repository) -> Self {
        let index_file_path = repo.gitdir.join(Self::INDEX_FILE);

        let index_file = File::open(&index_file_path).unwrap();
        let reader = BufReader::new(index_file);
        let mut entries: HashMap<String, StagingEntry> = HashMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(' ').collect();
            if parts.len() == 2 {
                let entry = StagingEntry {
                    path: PathBuf::from(parts[0]),
                    hash: parts[1].to_string(),
                };
                entries.insert(entry.path.to_str().unwrap().to_string(), entry);
            } else {
                panic!("Formato inválido no arquivo de índice");
            }
        }

        StagingArea { entries, repo }
    }

    /// Atualiza a entrada de um arquivo. Cria uma caso ela não exista
    pub fn update_or_create_entry(&mut self, file_path: &PathBuf) {
        let file = File::open(file_path).unwrap();
        let mut content: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut content).expect("Esperado ler o arquivo");
        
        let object = BlobObject::new(content);
        self.repo.create_object(&object);

        let new_entry = StagingEntry {
            path: file_path.clone(), 
            hash: object.hash()
        };
        
        self.entries.insert(file_path.to_str().unwrap().to_string(), new_entry);
    }

    /// Salva o estado da área de staging no arquivo de índice.
    /// 
    /// Este método consome a instância da área de staging, precisando criar uma nova.
    pub fn save(self) {
        let index_file_path = self.repo.gitdir.join(Self::INDEX_FILE);
        let mut index_file = File::create(index_file_path).expect("Esperado criar o arquivo de índice");
        let mut index_file_content: Vec<u8> = Vec::new();

        for (_key, entry) in self.entries {
            let mut entry_bytes = entry.as_bytes();
            entry_bytes.push(b'\n');
            index_file_content.append(&mut entry_bytes);
        }

        index_file.write_all(&index_file_content).expect("Esperado salvar o índice atual");
    }
}