use core::panic;
use std::{fs::{self, File}, path::{Path, PathBuf}};
use crate::{config::{GitConfig, RGitIgnore}, objects::{BlobObject, CommitObject, RGitObject, RGitObjectTypes, TreeObject}, staging::{StagingArea, StagingEntry}, utils::{is_valid_sha1, reference_exists, refs}};

/// Estrutura que representa o repositório do projeto
/// 
/// ## Atributos
/// - `worktree` - Caminho para a pasta raíz do repostitório
/// - `gitdir` - Caminho para a pasta .minigit do repositório
pub struct Repository{
    pub worktree: PathBuf,
    pub minigitdir: PathBuf,
    pub head_path: PathBuf,
    pub index_path: PathBuf,
    pub refs_heads_path: PathBuf,
    pub merge_head_path: PathBuf,
    pub orig_head_path: PathBuf,
    pub rebase_head_path: PathBuf,
    pub config: GitConfig
}

impl Repository {
    const MINIGITDIR : &'static str = ".minigit";
    const CONFIG : &'static str = "config";
    const HEAD : &'static str = "HEAD";
    const GITIGNORE : &'static str = ".gitignore";
    const INDEX : &'static str = "index";
    const MERGE_HEAD : &'static str = "MERGE_HEAD";
    const ORIG_HEAD : &'static str = "ORIG_HEAD";
    const REBASE_HEAD : &'static str = "REBASE_HEAD";

    pub fn new(path: &Path) -> Self {
        let minigit_path = path.join(Self::MINIGITDIR);
        let config_path = minigit_path.join(Self::CONFIG);
        let head_path = minigit_path.join(Self::HEAD);
        let index_path = minigit_path.join(Self::INDEX);
        let merge_head_path = minigit_path.join(Self::MERGE_HEAD);
        let orig_head_path = minigit_path.join(Self::ORIG_HEAD);
        let refs_heads_path = minigit_path.join("refs").join("heads");
        let rebase_head_path = minigit_path.join(Self::REBASE_HEAD);

        let config_bytes = std::fs::read(&config_path).unwrap_or_default();

        Repository {
            worktree: path.to_path_buf(),
            minigitdir: minigit_path,
            head_path: head_path,
            index_path: index_path,
            refs_heads_path: refs_heads_path,
            merge_head_path: merge_head_path,
            orig_head_path: orig_head_path,
            rebase_head_path: rebase_head_path,
            config: GitConfig::new(config_bytes)
        }
    }

    pub fn add_files(&mut self, relative_file_paths: Vec<PathBuf>) {
        let mut staging = StagingArea::new(self);
        let ignore = RGitIgnore::new(self);

        for relative_path in relative_file_paths {
            let absolute_path = self.worktree.join(&relative_path);

            if ignore.check_ignore(&relative_path) {
                continue;
            } else if absolute_path.exists() {
                let blob = BlobObject::from(&absolute_path);
                let hash = self.create_object(&blob);
                let last_content_change = fs::metadata(&absolute_path).unwrap().modified().unwrap()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos();
                
                let entry = StagingEntry {
                    last_content_change,
                    object_hash: hash,
                    path: relative_path,
                    mode_type: 0o100644, // arquivo normal
                };
                staging.update_or_create_entry(entry);
            } else {
                staging.remove_entry_with_path(&relative_path);
            }
        }
        
        std::fs::write(&self.index_path, staging.serialize()).unwrap();
    }

    /// Atualiza o arquivo de configuração do repositório
    pub fn update_config(&mut self, key: String, value: String) {
        self.config.set(key, value);
        let config_path = self.minigitdir.join(Self::CONFIG);
        std::fs::write(&config_path, &self.config.serialize()).unwrap();
    }

    /// Retorna o hash do commit apontado pelo HEAD do repositório
    pub fn resolve_head(&self) -> String {
        refs::resolve_head(self)
    }

    /// Verifica se a referência fornecida existe no repositório
    /// Referência pode ser o nome de uma branch ou um hash de commit direto
    /// 
    pub fn reference_exists(&self, reference: &String) -> bool {
        refs::reference_exists(reference, self)
    }

    /// Retorna o hash do commit apontado pela referência fornecida
    /// Referência pode ser o nome de uma branch ou um hash de commit direto
    /// 
    /// Esta função entra em pânico se a referência não existir. Verifique se a referência existe antes de chamar esta função.
    pub fn resolve_reference(&self, reference: &String) -> String {
        if !refs::reference_exists(reference, self) {
            panic!("Referência {} não existe!", reference);
        }

        refs::resolve_head_or_branch_name(reference, self).unwrap()
    }

    /// Verifica se o HEAD do repositório está destacado (é um hash de commit direto)
    pub fn head_detached(&self) -> bool {
        let head_string = std::fs::read_to_string(&self.head_path).unwrap();
        let head_string = head_string.trim();
        !head_string.starts_with("ref: ")
    }

    /// Retorna o nome da referência apontada pelo HEAD do repositório
    pub fn get_head(&self) -> String {
        let head_string = std::fs::read_to_string(&self.head_path).unwrap();

        if head_string.starts_with("ref: ") {
            head_string[5..].trim().to_string()
        } else {
            head_string.trim().to_string()
        }
    }

    /// Atualiza a branch atual para apontar para o novo commit
    /// Entra em pânico se o HEAD estiver destacado ou corrompido.
    pub fn update_curr_branch(&mut self, commit_id: &String) {
        let head_ref = self.get_head();

        if is_valid_sha1(&head_ref) {
            panic!("HEAD está destacado!");
        }

        self.update_branch_ref(&head_ref.to_string(), commit_id);
    }

    /// Atualiza a branch especificada para apontar para o novo commit
    /// Entra em pânico se a branch não existir
    pub fn update_branch(&mut self, branch_name: &String, commit_id: &String) {
        if !self.reference_exists(branch_name) || is_valid_sha1(branch_name) {
            panic!("Branch {} não existe!", branch_name);
        }

        let branch_ref_str = format!("refs/heads/{}", branch_name);
        self.update_branch_ref(&branch_ref_str, commit_id);
    }

    /// Atualiza a referência de branch especificada para apontar para o novo commit
    /// Entra em pânico se a referência não existir ou não estiver no formato '/refs/heads/...'
    pub fn update_branch_ref(&mut self, branch_ref: &String, commit_id: &String) {
        let branch_head = refs::resolve_ref_path(branch_ref, self);

        if branch_head.is_none() {
            panic!("Referência {} não existe!", branch_ref);
        }

        let branch_path_str = format!("{}/index", branch_ref);
        let parts = branch_path_str.split('/').collect::<Vec<&str>>();
        let branch_path = self.get_repository_path(parts.as_slice());
        std::fs::write(&branch_path, commit_id).unwrap();
    }

    pub fn update_head(&mut self, commit_id: &String) {
        let head_ref = self.get_head();
        let head_path = self.minigitdir.join(head_ref);

        std::fs::write(&head_path, commit_id).unwrap();
    }

    /// Muda o HEAD do repositório para o novo valor
    /// 
    /// `new_head` pode ser o hash de um commit ou o nome de uma branch existente.
    /// Certifique-se de que o valor passado é uma referência existente.
    pub fn change_head(&mut self, new_head: &String) {
        if !reference_exists(new_head, self) {
            panic!("Novo HEAD não é um commit ou uma branch válida");
        }

        let is_commit_id = is_valid_sha1(&new_head);
        let new_head_content = if is_commit_id {
            new_head.clone()
        } else {
            format!("ref: refs/heads/{}", new_head)
        };
        
        std::fs::write(&self.head_path, new_head_content).unwrap();
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

    pub fn get_commit_history(&self) -> Vec<CommitObject> {
        self.get_commit_history_from_commit(&self.resolve_head())
    }

    /// Retorna o histórico de commits do repositório a partir do commit fornecido
    /// Os commits estão ordenados do mais recente para o mais antigo
    /// Entra em pânico se o commit não existir
    fn get_commit_history_from_commit(&self, start_commit: &String) -> Vec<CommitObject> {
        let mut stack_of_ids: Vec<String> = Vec::new();
        let mut commit_history: Vec<CommitObject> = Vec::new();
        let head = start_commit.clone();

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
    /// 
    /// ## Retorna
    /// O hash do objeto criado
    pub fn create_object<T : RGitObject>(&mut self, object: &T) -> String {
        let hash = object.hash();
        let (dir, file_name) = hash.split_at(2);

        let path = self.get_repository_path(&["objects", dir, file_name]);

        if path.exists() {
            return hash;
        }

        self.create_repository_file(&["objects", dir, file_name]);
        std::fs::write(&path, object.get_object_bytes()).unwrap();

        hash
    }

    pub fn get_object(&self, object_id: &String) -> Option<RGitObjectTypes> {
        if object_id.is_empty() {
            return None;
        }

        let (dir, file_name) = object_id.split_at(2);
        let file_path = self.get_repository_path(&["objects", dir, file_name]);
        
        if !file_path.exists() {
            return None;
        }

        let (object_type, object_size, object_content) = Self::split_object_bytes(std::fs::read(&file_path).unwrap());

        if object_size != object_content.len() {
            panic!("Objeto foi corrompido!");
        }

        match object_type.as_str() {
            "commit" => {
                let commit = CommitObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Commit(commit))
            },
            "blob" => {
                let blob = BlobObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Blob(blob))
            },
            "tree" => {
                let tree = TreeObject::new(object_content.to_vec());
                Some(RGitObjectTypes::Tree(tree))
            },
            _ => {
                panic!("Tipo de objeto desconhecido!");
            }
        }
    }

    fn split_object_bytes(object_bytes: Vec<u8>) -> (String, usize, Vec<u8>) {
        let space = object_bytes.iter().position(|x| *x == b' ').unwrap();

        let (object_type, object_content) = object_bytes.split_at(space);
        let object_content = &object_content[1..];

        let null = object_content.iter().position(|x| *x == b'\0').unwrap();
        let (object_size, object_content) = object_content.split_at(null);
        let object_content = &object_content[1..];
        let object_size = std::str::from_utf8(object_size).unwrap().parse::<usize>().unwrap();

        if object_size != object_content.len() {
            panic!("Objeto foi corrompido!");
        }

        let object_type_str = std::str::from_utf8(object_type).unwrap().to_string();

        (object_type_str, object_size, object_content.to_vec())
    }

    pub fn clear_worktree(&mut self) {
        Self::clear_directory(&self.worktree, &RGitIgnore::new(self), self);
    }

    fn clear_directory(absolute_path: &PathBuf, ignore: &RGitIgnore, repo: &Repository) {
        for entry in std::fs::read_dir(absolute_path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();

            if entry_path.is_dir() {
                Self::clear_directory(&entry_path, ignore, repo);

                if entry_path.read_dir().unwrap().next().is_none() {
                    std::fs::remove_dir(&entry_path).unwrap();
                }
                
                continue;
            }

            let relative_path = entry_path.strip_prefix(&repo.worktree).unwrap();

            if !ignore.check_ignore(&relative_path.to_path_buf()) {
                std::fs::remove_file(&entry_path).unwrap();
            }
        }
    }

    pub fn delete_branch(&mut self, branch_name: &String) -> Result<(), String> {
        let branch_path_str = format!("refs/heads/{}", branch_name);
        let parts = branch_path_str.split('/').collect::<Vec<&str>>();
        let branch_path = self.get_repository_path(parts.as_slice());

        if !branch_path.exists() {
            return Err(String::from(format!("Branch {} não existe!", branch_name)));
        }

        let current_head = self.get_head();

        if current_head == format!("ref: refs/heads/{}", branch_name) {
            return Err(String::from("Não é possível deletar a branch atualmente ativa"));
        }

        std::fs::remove_file(&branch_path).map_err(|_| "Erro ao deletar a branch")?;

        Ok(())
    }

    pub fn create_branch(&mut self, branch_name: &String) -> Result<(), String> {
        let branch_path_str = format!("refs/heads/{}", branch_name);
        let parts = branch_path_str.split('/').collect::<Vec<&str>>();
        let branch_path = self.get_repository_path(parts.as_slice());

        if branch_path.exists() {
            return Err(String::from(format!("Branch {} já existe!", branch_name)));
        }

        let head_commit = self.resolve_head();
        let branch_path_parent = branch_path.parent().ok_or("Erro ao criar a branch")?;
        std::fs::create_dir_all(&branch_path_parent).map_err(|_| "Erro ao criar a branch")?;
        std::fs::write(&branch_path, head_commit).map_err(|_| "Erro ao criar a branch")?;

        Ok(())
    }
}

mod commands;
mod staging;
mod objects;
mod utils;
mod config;
mod status;
mod checks;

pub use commands::cli_main;
