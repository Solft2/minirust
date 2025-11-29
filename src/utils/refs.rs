use std::{path::PathBuf, str::FromStr};

use crate::{Repository};

/// Retorna o hash do commit referenciado pela `reference`
/// 
/// Pode ser uma referência direta (hash do commit) ou uma referência indireta (nome de uma branch)
/// 
/// - A referência pode ser o hash direto, 
/// - o nome da branch (ex: "main", "master") ou
/// - o caminho para referência (ex: "refs/heads/main") ou
/// - o caminho para a referência precedido de "ref: "
/// - A string HEAD
/// 
/// # Exemplos
/// - "refs/heads/master"
/// - "master"
/// - "ref: refs/heads/master"
/// - "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8"
/// - "HEAD"
pub fn resolve_ref(reference: &str, repo: &Repository) -> Option<String> {
    if reference == Repository::HEAD {
        let head_string = std::fs::read_to_string(&repo.head_path).unwrap();
        return resolve_ref(&head_string, repo);
    }

    if reference.starts_with("ref: ") {
        return resolve_ref_path(&reference[5..], repo);
    }

    if let Some(result) = resolve_ref_path(format!("refs/heads/{}", reference.trim()).as_str(), repo) {
        return Some(result);
    }

    let commit_id = reference.trim().to_string();
    let _ = repo.get_object(&commit_id)?;

    return Some(commit_id);
}

pub fn resolve_ref_path(reference: &str, repo: &Repository) -> Option<String> {
    let path = PathBuf::from_str(reference.trim()).ok()?;
    let full_path = repo.minigitdir.join(path);
    let ref_at_ref = std::fs::read_to_string(&full_path).ok()?;

    return resolve_ref(&ref_at_ref, repo);
}



