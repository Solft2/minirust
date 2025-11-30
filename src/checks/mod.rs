use crate::{Repository, status::{self, non_staged_files}, utils::merge_rebase};

/// Garante de forma segura que não há arquivos não adicionados no repositório
/// 
/// ## Retorna
/// - Ok(()): se não houver arquivos não adicionados
/// - Err(String): se houver arquivos não adicionados, com uma mensagem detalhando os arquivos.
pub fn ensure_no_non_staged_files(repo: &Repository) -> Result<(), String> {
    let non_staged_files= non_staged_files(repo);

    if !non_staged_files.is_empty() {
        let file_list = non_staged_files.iter()
            .map(|f| format!("\n- {}", f.to_str().unwrap()))
            .collect::<String>();

        let message = String::from("Existem arquivos não adicionados no repositório:\n") +
            "Adicione-os com 'minigit add <arquivos>' ou descarte as mudanças antes de continuar.\n" +
            "\nArquivos não adicionados:" +
            &file_list;

        return Err(message);
    }

    Ok(())
}

pub fn ensure_no_uncommited_changes(repo: &Repository) -> Result<(), String> {
    let uncommited_files = status::get_uncommited_files(repo);

    if !uncommited_files.is_empty() {
        let file_list = uncommited_files.iter()
            .map(|f| format!("\n- {}", f.to_str().unwrap()))
            .collect::<String>();

        let message = String::from("Existem arquivos com mudanças não commitadas no repositório:\n") +
            "Faça o commit dessas mudanças com 'minigit commit -m <mensagem>' ou descarte as mudanças antes de continuar.\n" +
            "\nArquivos com mudanças não commitadas:" +
            &file_list;

        return Err(message);
    }
    Ok(())
}

/// Garante de forma segura que não há um merge em progresso no repositório
/// 
/// ## Retorna
/// - Ok(()): se não houver merge em progresso
/// - Err(String): se houver um merge em progresso, com uma mensagem detalhando o problema.
pub fn ensure_no_merge_in_progress(repo: &Repository) -> Result<(), String> {
    if merge_rebase::is_in_progress(repo, false) {
        return Err("Há um merge em progresso. Finalize ou aborte antes de continuar.".to_string());
    }
    Ok(())
}

/// Garante de forma segura que há um merge em progresso no repositório
/// 
/// ## Retorna
/// - Ok(()): se houver um merge em progresso
/// - Err(String): se não houver um merge em progresso, com uma mensagem
pub fn ensure_merge_in_progress(repo: &Repository) -> Result<(), String> {
    if !merge_rebase::is_in_progress(repo, false) {
        return Err("Não há um merge em progresso para prosseguir.".to_string());
    }
    Ok(())
}

/// Garante de forma segura que não há um rebase em progresso no repositório
/// 
/// ## Retorna
/// - Ok(()): se não houver merge em progresso
/// - Err(String): se houver um merge em progresso, com uma mensagem detalhando o problema.
pub fn ensure_no_rebase_in_progress(repo: &Repository) -> Result<(), String> {
    if merge_rebase::is_in_progress(repo, true) {
        return Err("Há um rebase em progresso. Finalize ou aborte antes de continuar.".to_string());
    }
    Ok(())
}

/// Garante de forma segura que há um rebase em progresso no repositório
/// 
/// ## Retorna
/// - Ok(()): se houver um rebase em progresso
/// - Err(String): se não houver um rebase em progresso, com uma mensagem
pub fn ensure_rebase_in_progress(repo: &Repository) -> Result<(), String> {
    if !merge_rebase::is_in_progress(repo, true) {
        return Err("Não há um rebase em progresso para prosseguir.".to_string());
    }
    Ok(())
}

/// Garante de forma segura que o HEAD não está destacado no repositório
/// 
/// ## Retorna
/// - Ok(()): se o HEAD não estiver destacado
/// - Err(String): se o HEAD estiver destacado, com uma mensagem detalhando o problema.
pub fn ensure_no_detached_head(repo: &Repository) -> Result<(), String> {
    if repo.head_detached() {
        return Err("Não é possível realizar a operação com o HEAD destacado.".to_string());
    }
    Ok(())
}