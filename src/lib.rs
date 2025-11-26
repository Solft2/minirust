use core::panic;
use std::{fs::File, io::Write, path::{Path, PathBuf}};
use crate::objects::RGitObject;

/// Estrutura que representa o repositório do projeto
/// 
/// ## Atributos
/// - `worktree` - Caminho para a pasta raíz do repostitório
/// - `gitdir` - Caminho para a pasta .rgit do repositório
pub struct Repository{
    pub worktree: PathBuf,
    pub minigitdir: PathBuf,
}

impl Repository {
    const MINIGITDIR : &'static str = ".minigit";

    pub fn new(path: &Path) -> Self {
        Repository {
            worktree: path.to_path_buf(),
            minigitdir: path.join(Self::MINIGITDIR),
        }
    }

    /// Constroí um caminho de arquivo a partir da pasta .minigit do repositório
    /// 
    /// ## Argumentos
    /// - `parts` - As partes que formam o caminho
    /// 
    /// ## Exemplo
    /// ```
    /// get_repository_path(&["a", "b", "c"]) // .minigit/a/b/c
    /// ```
    pub fn get_repository_path(&self, parts: &[&str]) -> PathBuf {
        let mut path = self.minigitdir.clone();
        for p in parts {
            path.push(p);
        }
        path
    }

    /// Cria uma pasta no caminho especificado relativo ao .minigit
    /// 
    /// ## Argumentos
    /// - `parts` - Partes do caminho até a pasta
    pub fn create_repository_dir(&mut self, parts: &[&str]) {
        let path = self.get_repository_path(parts);
        std::fs::create_dir_all(&path).expect("Deveria criar o diretório");
    }

    /// Cria um arquivo no caminho especificado relativo ao .minigit
    /// 
    /// ## Argumentos
    /// - `parts` - Partes do caminho até o arquivo 
    pub fn create_repository_file(&mut self, parts: &[&str]) -> File {
        if parts.is_empty() {
            panic!("Foi tentado criar um arquivo sem nome")
        }

        self.create_repository_dir(&parts[0..parts.len()-1]);

        let path = self.get_repository_path(parts);

        File::create(path).expect("Deveria criar o arquivo")
    }
    

    /// Cria um objeto .minigit no repositório.
    /// Por questões de performance e organização, o objeto ficará em `.minigit/objects/<a>/<b>`, 
    /// onde `a` são os dois primeiros dígitos do hash e `b` é o restante do hash.
    /// 
    /// ## Argumentos
    /// - `object` - O objeto RGit 
    pub fn create_object<T : RGitObject>(&mut self, object: &T) {
        let hash = object.hash();
        let (dir, file_name) = hash.split_at(2);

        let path = self.get_repository_path(&["objects", dir, file_name]);

        if path.exists() {
            return;
        }

        let mut file = self.create_repository_file(&["objects", dir, file_name]);
        file.write_all(&object.serialize()).expect("Deveria escrever o conteúdo do objeto");
    }
}

pub mod commands;
pub mod staging;
pub mod objects;
pub mod utils;

pub use commands::cli_main;
