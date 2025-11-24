use std::{fs::File, io::Write, path::{Path, PathBuf}};
use crate::objects::RGitObject;

/// Estrutura que representa o repositório do projeto
/// 
/// ## Atributos
/// - `worktree` - Caminho para a pasta raíz do repostitório
/// - `gitdir` - Caminho para a pasta .rgit do repositório
pub struct Repository{
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
}

impl Repository {
    const RGITDIR : &'static str = ".rgit";

    pub fn new(path: &Path) -> Self {
        Repository {
            worktree: path.to_path_buf(),
            gitdir: path.join(Self::RGITDIR),
        }
    }

    /// Constroí um caminho de arquivo a partir da pasta .rgit do repositório
    /// 
    /// ## Argumentos
    /// - `partes` - As partes que formam o caminho
    /// 
    /// ## Exemplo
    /// ```
    /// repository_path(&["a", "b", "c"]) // .rgit/a/b/c
    /// ```
    pub fn repository_path(&self, partes: &[&str]) -> PathBuf {
        let mut path = self.gitdir.clone();
        for p in partes {
            path.push(p);
        }
        path
    }

    /// Retorna o caminho para a pasta .rgit se ela estiver bem configurada.
    /// Caso contrário, retorna `None`.
    pub fn get_rgitdir(&self) -> Option<PathBuf> {
        if self.gitdir.exists() && self.gitdir.is_dir() {
            Some(self.gitdir.clone())
        } else {
            None
        }
    }

    /// Retorna um caminho para um diretório a partir da pasta .rgit
    /// 
    /// ## Argumentos
    /// - `partes` - Partes do caminho do diretório
    /// - `mkdir` - Flag para criar os diretórios intermediários que não existem
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

    /// Retorna um caminho para um arquivo a partir da pasta .rgit
    /// 
    /// ## Argumentos
    /// - `partes` - Partes do caminho do arquivo
    /// - `mkdir` - Flag para criar os diretórios intermediários que não existem
    pub fn repository_file(&self, parts: &[&str], mkdir: bool) -> PathBuf {
        if let Some(_) = self.repository_dir(&parts[..parts.len() - 1], mkdir) {
            self.repository_path(parts)
        } else {
            panic!("Não foi possível criar o diretório do arquivo!");
        }
    }

    /// Cria um objeto .rgit no repositório.
    /// Por questões de performance e organização, o objeto ficará em `.rgit/objects/<a>/<b>`, 
    /// onde `a` são os dois primeiros dígitos do hash e `b` é o restante do hash.
    /// 
    /// ## Argumentos
    /// - `object` - O objeto RGit 
    pub fn create_object<T : RGitObject>(&mut self, object: &T) {
        let hash = object.hash();
        let (dir, file_name) = hash.split_at(2);

        let path = self.repository_file(&["objects", dir, file_name], true);

        let mut file = File::create(path).expect("Deveria criar o objeto");
        file.write_all(&object.serialize()).expect("Deveria escrever o conteúdo do objeto");
    }
}

pub mod commands;
pub mod staging;
pub mod objects;
pub mod utils;

pub use commands::cli_main;
