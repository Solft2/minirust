use std::{path::PathBuf, str::FromStr};

use crate::{Repository, objects::{RGitObjectTypes}, utils::is_valid_sha1};

/// Retorna o hash do commit referenciado pela `reference`
/// Referência pode ser:
/// - HEAD
/// - nome de branch (ex: "main", "master")
/// 
/// A função retorna None se a referência não existe.
/// Esta função pode dar pânico se pasta .minigit estiver corrompida.
pub fn resolve_head_or_branch_name(reference: &str, repo: &Repository) -> Option<String> {
    if reference == Repository::HEAD {
        return Some(resolve_head(repo));
    }

    return resolve_ref_path(format!("refs/heads/{}", reference).as_str(), repo);
}


/// Retorna o hash do commit apontado por HEAD
/// 
/// HEAD pode ter o seguinte formato:
/// - ref: refs/heads/main
/// - hash do commit diretamente
///
/// HEAD SEMPRE deve apontar para um commit válido. Caso contrário, a função dará pânico.
pub fn resolve_head(repo: &Repository) -> String {
    let head_string = std::fs::read_to_string(&repo.head_path).unwrap();
    let head_string = head_string.trim();
    let is_commit = !head_string.starts_with("ref: ");

    if is_commit {
        if !is_valid_sha1(head_string) {
            panic!("HEAD contém uma referência inválida");
        }

        let object = repo.get_object(&head_string.to_string());

        match object {
            None => panic!("HEAD contém uma referência inválida"),
            Some(obj) => match obj {
                RGitObjectTypes::Commit(_) => {
                    return head_string.to_string();
                }
                _ => panic!("HEAD contém uma referência inválida"),
            },
        }
    }

    let reference = &head_string[5..]; // remove "ref: "
    match resolve_ref_path(reference, repo) {
        None => panic!("HEAD contém uma referência inválida"),
        Some(commit_id) => commit_id,
    }
}

/// Retorna o hash do commit referenciado pela `reference` em 'refs/heads/.../index'
/// 
/// Retorna None se a referência não existir.
/// Assumimos que as referências sempre apontam para um commit e que o caminho começa com "refs/heads/".
/// Caso a referência exista e não aponte para nada, isso é considerado válido e retornamos uma string vazia.
pub fn resolve_ref_path(reference: &str, repo: &Repository) -> Option<String> {
    if !reference.starts_with("refs/heads/") {
        panic!("resolve_ref_path deve ser chamado apenas com referências que começam com 'refs/heads/'");
    }

    let path = PathBuf::from_str(reference.trim()).ok()?;
    let branch_index_path = path.join("index");
    let full_path = repo.minigitdir.join(branch_index_path);
    let commit_at_ref = std::fs::read_to_string(&full_path).ok()?;

    if commit_at_ref.trim().is_empty() {
        return Some(String::new());
    }

    let commit_id = commit_at_ref.trim().to_string();
    let obj = repo.get_object(&commit_id);

    match obj {
        None => panic!("A referência '{}' aponta para um objeto inválido", reference),
        Some(o) => match o {
            RGitObjectTypes::Commit(_) => Some(commit_id),
            _ => panic!("A referência '{}' não aponta para um commit", reference),
        },
    }
}

/// Verifica se a referência passada existe
pub fn reference_exists(reference: &str, repo: &Repository) -> bool {
    if is_valid_sha1(reference) {
        let object = repo.get_object(&reference.to_string());
        return match object {
            Some(obj) => match obj {
                RGitObjectTypes::Commit(_) => true,
                _ => false,
            },
            None => false,
        };
    }

    resolve_head_or_branch_name(reference, repo).is_some()
}



