use core::panic;
use std::{fs::File, io::{BufReader, Read, Write}, path::{Path, PathBuf}};
use crate::{objects::{BlobObject, CommitObject, RGitObject, RGitObjectTypes, TreeObject}, utils::{read_string_from_file, resolve_ref, write_string_to_file}};

/// Estrutura que representa o repositório do projeto
/// 
/// ## Atributos
/// - `worktree` - Caminho para a pasta raíz do repostitório
/// - `gitdir` - Caminho para a pasta .rgit do repositório
pub struct Repository{
    pub worktree: PathBuf,
    pub minigitdir: PathBuf,
    pub head_path: PathBuf,
}

impl Repository {
    const MINIGITDIR : &'static str = ".minigit";

    pub fn new(path: &Path) -> Self {
        Repository {
            worktree: path.to_path_buf(),
            minigitdir: path.join(Self::MINIGITDIR),
            head_path: path.join(Self::MINIGITDIR).join("HEAD"),
        }
    }

    /// Retorna o hash do commit apontado pelo HEAD do repositório
    pub fn resolve_head(&self) -> String {
        let ref_string = read_string_from_file(&self.head_path);
        resolve_ref(&ref_string, self)
    }

    /// Retorna o nome da referência apontada pelo HEAD do repositório
    pub fn get_head(&self) -> String {
        let head_string = read_string_from_file(&self.head_path); 

        if head_string.starts_with("ref: ") {
            head_string[5..].trim().to_string()
        } else {
            head_string.trim().to_string()
        }
    }

    pub fn update_head(&mut self, commit_id: &String) {
        let head_ref = self.get_head();
        let head_path = self.minigitdir.join(head_ref);

        write_string_to_file(&head_path, commit_id);
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

    /// Retorna o histórico de commits do repositório, começando do HEAD
    pub fn get_commit_history(&self) -> Vec<CommitObject> {
        let mut stack_of_ids: Vec<String> = Vec::new();
        let mut commit_history: Vec<CommitObject> = Vec::new();
        let head = self.resolve_head();
        if head.is_empty() {
            return commit_history;
        }

        stack_of_ids.push(head);

        while !stack_of_ids.is_empty() {
            let current_commit_id = stack_of_ids.pop().unwrap();
            let object = self.get_object(&current_commit_id);
            if let Some(RGitObjectTypes::Commit(commit)) = object {
                for parent in &commit.parent {
                    stack_of_ids.push(parent.clone());
                }
                commit_history.push(commit);
            } else {
                break;
            }
        }

        commit_history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        commit_history
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
        file.write_all(&object.get_object_bytes()).expect("Deveria escrever o conteúdo do objeto");
    }

    pub fn get_object(&self, object_id: &String) -> Option<RGitObjectTypes> {
        let (dir, file_name) = object_id.split_at(2);
        let file_path = self.get_repository_path(&["objects", dir, file_name]);
        
        if !file_path.exists() {
            return None;
        }

        let file = File::open(file_path).unwrap();
        let mut content: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(file);

        reader.read_to_end(&mut content).unwrap();
        let space = content.iter().position(|x| *x == b' ').unwrap();

        let (object_type, object_content) = content.split_at(space);
        let object_content = &object_content[1..];

        match object_type {
            b"commit" => {
                let commit = CommitObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Commit(commit))
            },
            b"blob" => {
                let blob = BlobObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Blob(blob))
            },
            b"tree" => {
                let tree = TreeObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Tree(tree))
            },
            _ => {
                panic!("Tipo de objeto desconhecido!");
            }
        }
    }
}

pub mod commands;
pub mod staging;
pub mod objects;
pub mod utils;

pub use commands::cli_main;
