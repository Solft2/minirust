use crate::Repository;
use crate::commands::checkout::cmd_checkout;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum ResetTypes {
    Soft,
    Mixed,
    Hard,
}

pub fn cmd_reset(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("Uso correto: minigit reset <modo> <commit-id>");
        eprintln!("Modos disponíveis: soft, mixed, hard");
        return;
    }

    let mode_str = &args[0];
    let commit_id = &args[1];

    let mode = match mode_str.as_str() {
        "soft" => ResetTypes::Soft,
        "mixed" => ResetTypes::Mixed,
        "hard" => ResetTypes::Hard,
        _ => {
            eprintln!("Modo inválido! Use soft, mixed ou hard.");
            return;
        }
    };

    // CARREGA REPOSITÓRIO CORRETAMENTE
    let path = Path::new(".");
    let mut repo = Repository::new(path);

    // CHAMA A LÓGICA DO RESET
    if let Err(e) = reset(&mut repo, commit_id, mode) {
        eprintln!("Erro executando reset: {}", e);
    }
}

pub fn reset(repo: &mut Repository, target_hash: &str, mode: ResetTypes) -> Result<(), String> {
    repo.update_head(&target_hash.to_string());

    match mode {
        ResetTypes::Soft => {
            println!("Soft reset feito para {}", target_hash);
        }

        ResetTypes::Mixed => {
            let index_path = repo.get_repository_path(&["index"]);
            if index_path.exists() {
                fs::write(&index_path, "").unwrap();
            }
            println!("Mixed reset feito para {}", target_hash);
        }

        ResetTypes::Hard => {
            let index_path = repo.get_repository_path(&["index"]);
            if index_path.exists() {
                fs::write(&index_path, "").unwrap();
            }

            let commit_id = target_hash.to_string();
            cmd_checkout(&commit_id);

            println!("Hard reset feito para {}", target_hash);
        }
    }

    Ok(())
}
