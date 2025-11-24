use core::{panic};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::{collections::HashMap, path::PathBuf};
use crate::Repository;
use crate::objects::{BlobObject, RGitObject};

pub struct StagingEntry {
    pub path: PathBuf,
    pub hash: String
}

impl StagingEntry {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut entry_str: String = String::new();
        entry_str.push_str(self.path.to_str().unwrap());
        entry_str.push(' ');
        entry_str.push_str(&self.hash);

        entry_str.as_bytes().to_vec()
    }
}

pub struct StagingArea<'a> {
    pub entries: HashMap<String, StagingEntry>,
    pub repo: &'a mut Repository
}

impl<'a> StagingArea<'a> {
    const INDEX_FILE: &'static str = "index";

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

    pub fn save(self) {
        let index_file_path = self.repo.gitdir.join(Self::INDEX_FILE);
        let mut index_file = File::create(index_file_path).expect("Esperado criar o arquivo de índice");
        let mut index_file_content: Vec<u8> = Vec::new();

        for (_key, entry) in self.entries {
            let mut entry_bytes = entry.as_bytes();
            index_file_content.append(&mut entry_bytes);
        }

        index_file.write_all(&index_file_content).expect("Esperado salvar o índice atual");
    }
}