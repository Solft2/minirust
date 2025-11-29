use std::{path::PathBuf, str::FromStr};

use crate::{Repository};

/// Retorna o hash do commit referenciado pela `reference`
/// 
/// Pode ser uma referência direta (hash do commit) ou uma referência indireta (nome de uma branch)
/// 
/// - A referência pode ser o hash direto, 
/// - o caminho para referência (ex: "refs/heads/main") ou
/// - o caminho para a referência precedido de "ref: "
/// 
/// # Exemplos
/// - "refs/heads/master"
/// - "ref: refs/heads/master"
/// - "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8"
pub fn resolve_ref(reference: &str, repo: &Repository) -> String {
    if reference.starts_with("ref: ") {
        return resolve_ref(&reference[5..], repo);
    }

    if reference.contains("/") {
        let path = PathBuf::from_str(reference.trim()).unwrap();
        let full_path = repo.minigitdir.join(path);
        return std::fs::read_to_string(&full_path).unwrap();
    }

    return reference.trim().to_string();
}



