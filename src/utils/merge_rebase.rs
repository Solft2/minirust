use core::panic;
use std::fs;

use crate::{Repository, commands::checkout, objects::RGitObjectTypes};

/// Verifica se há um merge ou rebase em progresso no repositório
/// 
/// ## Argumentos
/// - is_rebase: bool - true para rebase, false para merge
pub fn is_in_progress(repo: &Repository, is_rebase: bool) -> bool {
    if is_rebase {
        repo.rebase_head_path.exists()
    } else {
        repo.merge_head_path.exists()
    }
}

/// Aborta um merge ou rebase em progresso no repositório
/// 
/// ## Panics
/// Entra em pânico se não houver um merge ou rebase em progresso. 
/// Isso precisa ser verificado antes de chamar essa função.
pub fn abort(repo: &mut Repository, is_rebase: bool) {
    if !is_in_progress(repo, is_rebase) {
        panic!("Não há um merge ou rebase em progresso para abortar.");
    }

    let original_commit = fs::read_to_string(&repo.orig_head_path).unwrap();
    let RGitObjectTypes::Commit(commit_object) = repo.get_object(&original_commit).unwrap()
        else { panic!("Commit original inválido") };

    repo.update_branch_ref(&repo.get_head(), &original_commit);

    repo.clear_worktree();
    checkout::instanciate_commit(commit_object, repo);

    finish(repo, is_rebase);

    println!("{} abortado com sucesso.", if is_rebase { "Rebase" } else { "Merge" });
}

/// Inicia um merge ou rebase, criando os arquivos de controle necessários
/// 
/// ## Panics
/// Entra em pânico se já houver um merge ou rebase em progresso.
/// Isso precisa ser verificado antes de chamar essa função.
pub fn start(repo: &mut Repository, is_rebase: bool) {
    if is_in_progress(repo, is_rebase) {
        panic!("Já há um merge ou rebase em progresso.");
    }

    let head_path = if is_rebase {
        &repo.rebase_head_path
    } else {
        &repo.merge_head_path
    };
    fs::write(&repo.orig_head_path, &repo.resolve_head()).unwrap();
    fs::write(head_path, "").unwrap();
}

/// Finaliza um merge ou rebase em progresso, removendo os arquivos de controle
/// Nada acontece se não houver um merge ou rebase em progresso.
pub fn finish(repo: &mut Repository, is_rebase: bool) {
    if is_in_progress(repo, is_rebase) {
        if is_rebase {
            fs::remove_file(&repo.rebase_head_path).unwrap();
        } else {
            fs::remove_file(&repo.merge_head_path).unwrap();
        }
        fs::remove_file(&repo.orig_head_path).unwrap();
    }
}

/// Retorna os commits que ainda faltam ser mergidos em um merge ou rebase em progresso.
/// 
/// ## Panics
/// Entra em pânico se não houver um merge ou rebase em progresso.
/// Isso precisa ser verificado antes de chamar essa função.
pub fn get_state(repo: &Repository, is_rebase: bool) -> Vec<String> {
    if is_in_progress(repo, is_rebase) {
        let head_path = if is_rebase {
            &repo.rebase_head_path
        } else {
            &repo.merge_head_path
        };

        let head_content = fs::read_to_string(head_path).unwrap();
        head_content
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        panic!("Não há um merge ou rebase em progresso.");
    }
}