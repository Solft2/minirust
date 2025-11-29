use crate::Repository;
use crate::commands::checkout::cmd_checkout;
use std::fs;

/// Tipos de reset
#[derive(Debug, Clone, Copy)]
pub enum ResetTypes {
    Soft,
    Mixed,
    Hard,
}

/// Realiza o reset de acordo com o tipo
pub fn reset(repo: &mut Repository, target_hash: &str, mode: ResetTypes) -> Result<(), String> {
    // Atualiza o HEAD para o commit alvo
    repo.update_head(&target_hash.to_string());

    match mode {
        ResetTypes::Soft => {
            // Soft: só atualiza o HEAD, não mexe no index nem no working directory
            println!("Soft reset feito para {}", target_hash);
        }
        ResetTypes::Mixed => {
            // Mixed: limpa apenas a staging area (index)
            let index_path = repo.get_repository_path(&["index"]);
            if index_path.exists() {
                fs::remove_file(&index_path)
                    .map_err(|e| format!("Erro removendo index: {}", e))?;
            }
            println!("Mixed reset feito para {}", target_hash);
        }
        ResetTypes::Hard => {
            // Hard: limpa index e atualiza working directory
            let index_path = repo.get_repository_path(&["index"]);
            if index_path.exists() {
                fs::remove_file(&index_path)
                    .map_err(|e| format!("Erro removendo index: {}", e))?;
            }

            // Atualiza arquivos do working tree para o commit
            // Passamos referência para cmd_checkout, que espera &String
            let commit_id = target_hash.to_string();
            cmd_checkout(&commit_id);

            println!("Hard reset feito para {}", target_hash);
        }
    }

    Ok(())
}
