use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{path::PathBuf};

use crate::Repository;
use crate::objects::{CommitObject, RGitObjectTypes, tree};

/// Uma entrada da staging area
#[derive(Debug, Clone)]
pub struct StagingEntry {
    pub last_content_change: u128, //in nanos
    pub mode_type: u32,
    pub object_hash: String,
    pub path: PathBuf, // caminho relativo ao worktree
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
            last_content_change: parse_to::<u128>(parts[0]), 
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
pub struct StagingArea {
    pub entries: Vec<StagingEntry>,
}

impl StagingArea {
    /// Carrega a área de staging do arquivo `index`
    pub fn new(repo: &Repository) -> Self {
        let index_file_path = repo.minigitdir.join(Repository::INDEX);

        let index_file = File::open(&index_file_path).unwrap();
        let reader = BufReader::new(index_file);
        let mut entries: Vec<StagingEntry> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            entries.push(StagingEntry::from_string(line));
        }

        StagingArea { entries }
    }

    /// Atualiza a entrada de um arquivo. Cria uma caso ela não exista
    pub fn update_or_create_entry(&mut self, entry: StagingEntry) {
        let curr_entry = self.entries.iter().position(|e| e.path == entry.path);

        if let Some(position) = curr_entry {
            self.entries[position] = entry;
        } else {
            self.entries.push(entry);
        }
    }

    pub fn remove_entry_with_path(&mut self, path: &PathBuf) {
        self.entries.retain(|e| &e.path != path);
    }

    /// Serializa a área de staging de volta para o arquivo `index`
    pub fn serialize(&self) -> Vec<u8> {
        let mut index_file_content: Vec<u8> = Vec::new();

        for entry in &self.entries {
            let mut entry_bytes = entry.as_bytes();
            entry_bytes.push(b'\n');
            index_file_content.append(&mut entry_bytes);
        }

        index_file_content
    }

    pub fn get_entries_as_map(&self) -> HashMap<PathBuf, StagingEntry> {
        let mut entries_map: HashMap<PathBuf, StagingEntry> = HashMap::new();
        for entry in self.entries.clone() {
            entries_map.insert(entry.path.clone(), entry);
        }
        entries_map
    }
}

pub fn rewrite_index_from_commit_id(repo: &mut Repository, commit_id: &String) {
    let RGitObjectTypes::Commit(commit_object) = repo
        .get_object(commit_id)
        .unwrap()
        else { panic!("ID de commit inválido ao reescrever índice"); };

    let staging_area = staging_area_from_commit(repo, &commit_object);
    rewrite_index(repo, &staging_area);
}

pub fn rewrite_index(repo: &mut Repository, staging_area: &StagingArea) {
    let index_file_path = repo.minigitdir.join(Repository::INDEX);
    std::fs::write(&index_file_path, staging_area.serialize()).unwrap();
}

fn parse_to<T: FromStr>(s: &str) -> T 
    where
        <T as FromStr>::Err: Debug
{
    str::parse::<T>(s).expect("Deveria ser possível o parsing")
}

pub fn staging_area_from_commit(repository: &Repository, commit: &CommitObject) -> StagingArea {
    let tree_hash = &commit.tree;
    let RGitObjectTypes::Tree(tree_object) = repository
        .get_object(tree_hash)
        .unwrap()
        else { panic!("Commit aponta para árvore inválida"); };

    let tree_files = tree::get_tree_as_map(repository, &tree_object);
    let relative_file_paths: Vec<String> = tree_files.keys().cloned().collect();

    StagingArea {
        entries: relative_file_paths.iter().map(|path| {
            let full_path = repository.worktree.join(path);
            let object_hash = tree_files.get(path).unwrap().clone();
            let relative_path = full_path.strip_prefix(&repository.worktree).unwrap();

            StagingEntry {
                last_content_change: commit.timestamp,
                mode_type: 0o100644, // arquivo normal
                object_hash: object_hash.to_string(),
                path: relative_path.to_path_buf(),
            }
        }).collect()
    }
}