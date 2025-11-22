use std::path::{Path, PathBuf}; // path caminho imutavel pathbuf caminho mutavel

pub struct Repository{
    pub worktree: PathBuf, // pasta geral
    pub gitdir: PathBuf,  //pasta .git
}


// Exemplo: Repository::new("meu_projeto") 
// Aponta para: worktree = "meu_projeto"/gitdir = "meu_projeto/.git"
impl Repository {
    pub fn new(path: &Path) -> Self {
        Repository {
            worktree: path.to_path_buf(),
            gitdir: path.join(".git"),
        }
    }
//Exemplo: repo.repository_path(&["objects", "ab", "c123ef"])
//Transforma em = meu_projeto/.git/objects/ab/c123ef
    pub fn repository_path(&self, partes: &[&str]) -> PathBuf {
        let mut path = self.gitdir.clone();
        for p in partes {
            path.push(p);
        }
        path
    }


//
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
}
pub mod commands;

pub use commands::cli_main;
