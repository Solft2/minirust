use std::{fs::File, io::Write, path::{Path, PathBuf}}; // path caminho imutavel pathbuf caminho mutavel
use crate::objects::RGitObject;

pub struct Repository{
    pub worktree: PathBuf, // pasta geral
    pub gitdir: PathBuf,  //pasta .git
}

impl Repository {
    const RGITDIR : &'static str = ".rgit";

    pub fn new(path: &Path) -> Self {
        Repository {
            worktree: path.to_path_buf(),
            gitdir: path.join(Self::RGITDIR),
        }
    }

    pub fn repository_path(&self, partes: &[&str]) -> PathBuf {
        let mut path = self.gitdir.clone();
        for p in partes {
            path.push(p);
        }
        path
    }

    pub fn get_rgitdir(&self) -> Option<PathBuf> {
        if self.gitdir.exists() && self.gitdir.is_dir() {
            Some(self.gitdir.clone())
        } else {
            None
        }
    }

    pub fn repository_dir(&self, partes: &[&str], mkdir: bool) -> Option<PathBuf> {
        let path = self.repository_path(partes);
        if path.exists() {
            if path.is_dir() {
                return Some(path);
            }
            panic!("Não é diretorio: {:?}", path);
        }
        if mkdir {
            std::fs::create_dir_all(&path).unwrap();
            return Some(path);
        }
        None
    }
//garante que o repositorio existe e retorna o caminho do arquivo
    pub fn repository_file(&self, parts: &[&str], mkdir: bool) -> PathBuf {
        if let Some(_) = self.repository_dir(&parts[..parts.len() - 1], mkdir) {
            self.repository_path(parts)
        } else {
            panic!("Não foi possível criar o diretório do arquivo!");
        }
    }

    pub fn create_object<T : RGitObject>(&mut self, object: &T) {
        let hash = object.hash();
        let (dir, file_name) = hash.split_at(2);

        let path = self.repository_file(&["objects", dir, file_name], true);

        let mut file = File::create(path).expect("Erro ao criar o objeto.");
        file.write_all(&object.serialize()).expect("Erro ao escrever no arquivo.");
    }
}

pub mod commands;
pub mod staging;
pub mod objects;
pub mod utils;

pub use commands::cli_main;
