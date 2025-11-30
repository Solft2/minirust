use core::panic;
use std::fs;

use crate::{Repository, commands::checkout::instanciate_commit};

/// Verifica se há um merge ou rebase em progresso no repositório
pub fn merge_or_rebase_in_progress(repo: &Repository) -> bool {
    repo.merge_head_path.exists()
}

/// Aborta um merge ou rebase em progresso no repositório
/// 
/// ## Panics
/// Entra em pânico se não houver um merge ou rebase em progresso. 
/// Isso precisa ser verificado antes de chamar essa função.
pub fn abort_merge(repo: &mut Repository) {
    if !merge_or_rebase_in_progress(repo) {
        panic!("Não há um merge ou rebase em progresso para abortar.");
    }

    let original_commit_hash = fs::read_to_string(&repo.orig_head_path)
        .expect("Arquivo ORIG_HEAD deveria existir")
        .trim()
        .to_string();

    repo.update_curr_branch(&original_commit_hash);

    repo.clear_worktree();

    let original_commit_obj = repo.get_object(&original_commit_hash)
        .expect("Commit original sumiu da pasta .minigit");

    instanciate_commit(original_commit_obj, repo);

    finish_merge(repo);

    println!("Merge/Rebase abortado com sucesso.");
}

/// Inicia um merge ou rebase, criando os arquivos de controle necessários
/// 
/// ## Panics
/// Entra em pânico se já houver um merge ou rebase em progresso.
/// Isso precisa ser verificado antes de chamar essa função.
pub fn start_merge(repo: &mut Repository) {
    if merge_or_rebase_in_progress(repo) {
        panic!("Já há um merge ou rebase em progresso.");
    }

    fs::write(&repo.orig_head_path, &repo.resolve_head()).unwrap();
    fs::write(&repo.merge_head_path, "").unwrap();
}

/// Finaliza um merge ou rebase em progresso, removendo os arquivos de controle
/// Nada acontece se não houver um merge ou rebase em progresso.
pub fn finish_merge(repo: &mut Repository) {
    if merge_or_rebase_in_progress(repo) {
        fs::remove_file(&repo.merge_head_path).unwrap();
        fs::remove_file(&repo.orig_head_path).unwrap();
    }
}

/// Retorna os commits que ainda faltam ser mergidos em um merge ou rebase em progresso.
/// 
/// 
/// ## Panics
/// Entra em pânico se não houver um merge ou rebase em progresso.
/// Isso precisa ser verificado antes de chamar essa função.
pub fn merge_state(repo: &Repository) -> Vec<String> {
    if merge_or_rebase_in_progress(repo) {
        let merge_head_content = fs::read_to_string(&repo.merge_head_path).unwrap();
        merge_head_content
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        panic!("Não há um merge ou rebase em progresso.");
    }
}