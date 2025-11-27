use std::fmt::Debug;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::MetadataExt;
use std::str::FromStr;
use std::time::UNIX_EPOCH;
use std::{path::PathBuf};
use crate::Repository;
use crate::objects::{BlobObject, RGitObject};

/// Uma entrada da staging area
pub struct StagingEntry {
    pub last_content_change: u64, //seconds
    pub mode_type: u32,
    pub object_hash: String,
    pub path: PathBuf,
}

impl StagingEntry {
    /// Converte a entrada de staging em um array de bytes para ser escrito no arquivo de índice
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut entry_str: String = String::new();

        entry_str.push_str(&self.last_content_change.to_string());
        entry_str.push(' ');

        entry_str.push_str(&self.mode_type.to_string());
        entry_str.push(' ');

        entry_str.push_str(&self.object_hash.to_string());
        entry_str.push(' ');

        entry_str.push_str(self.path.to_str().unwrap());

        entry_str.as_bytes().to_vec()
    }

    pub fn from_string(s: String) -> Self {
        let parts: Vec<&str> = s.split(' ').collect();

        StagingEntry { 
            last_content_change: parse_to::<u64>(parts[0]), 
            mode_type: parse_to::<u32>(parts[1]), 
            object_hash: parts[2].to_string(), 
            path: PathBuf::from(parts[3])
        }
    }
}

/// Representa a área de staging, onde os arquivos alterados vão antes de serem comitados.
/// 
/// A implementação é através de um arquivo que mantém um mapeamento de caminho -> hash do objeto.
/// 
/// Quando o usuário adicionar um arquivo em staging (comando add), devemos atualizar a entrada correspondente
pub struct StagingArea<'a> {
    pub entries: Vec<StagingEntry>,
    pub repo: &'a mut Repository
}

impl<'a> StagingArea<'a> {
    const INDEX_FILE: &'static str = "index";

    /// Carrega a área de staging do arquivo `index`
    pub fn new(repo: &'a mut Repository) -> Self {
        let index_file_path = repo.minigitdir.join(Self::INDEX_FILE);

        let index_file = File::open(&index_file_path).unwrap();
        let reader = BufReader::new(index_file);
        let mut entries: Vec<StagingEntry> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            entries.push(StagingEntry::from_string(line));
        }

        StagingArea { entries, repo }
    }

    /// Atualiza a entrada de um arquivo. Cria uma caso ela não exista
    pub fn update_or_create_entry(&mut self, file_path: &PathBuf) {
        if !file_path.exists() {
            return;
        }

        let curr_entry = self.entries.iter().find(|entry| {
            entry.path == *file_path
        });

        let file = File::open(file_path).unwrap();
        let file_metadata = fs::metadata(file_path).unwrap();
        let last_content_change = file_metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(entry) = curr_entry {
            if entry.last_content_change == last_content_change {
                return;
            }
        }

        let mut content: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut content).expect("Esperado ler o arquivo");
        
        let object = BlobObject::new(content);
        self.repo.create_object(&object);

        let new_entry = StagingEntry {
            path: file_path.clone(), 
            object_hash: object.hash(),
            mode_type: file_metadata.mode(),
            last_content_change: last_content_change
        };

        if let Some(entry) = curr_entry {
            let position = self.entries.iter().position(|e| e.path == *entry.path).unwrap();

            self.entries[position] = new_entry
        } else {
            self.entries.push(new_entry);
        }
    }

    /// Salva o estado da área de staging no arquivo de índice.
    /// 
    /// Este método consome a instância da área de staging, precisando criar uma nova.
    pub fn save(self) {
        let index_file_path = self.repo.minigitdir.join(Self::INDEX_FILE);
        let mut index_file = File::create(index_file_path).expect("Esperado criar o arquivo de índice");
        let mut index_file_content: Vec<u8> = Vec::new();

        for entry in self.entries {
            let mut entry_bytes = entry.as_bytes();
            entry_bytes.push(b'\n');
            index_file_content.append(&mut entry_bytes);
        }

        index_file.write_all(&index_file_content).expect("Esperado salvar o índice atual");
    }
}

fn parse_to<T: FromStr>(s: &str) -> T 
    where
        <T as FromStr>::Err: Debug
{
    str::parse::<T>(s).expect("Deveria ser possível o parsing")
}