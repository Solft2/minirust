use crate::utils::{find_current_repo, reference_exists};

pub fn cmd_rebase(continue_: bool, new_base_branch: Option<String>) {
    match cmd_rebase_result(continue_, new_base_branch) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_rebase_result(continue_: bool, new_base_branch: Option<String>) -> Result<(), String> {
    let repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    if continue_ {
        continue_rebase()?;
    } else if let Some(branch) = new_base_branch {
        let exists = reference_exists(&branch, &repo);

        if !exists {
            return Err("A referência fornecida não existe.".to_string());
        }

        start_rebase(branch)?;
    } else {
        return Err("Você deve fornecer nova base de branch para rebase.".to_string());
    }

    Ok(())
}

fn continue_rebase() -> Result<(), String> {
    // Lógica para continuar o rebase
    println!("Continuando o rebase...");
    Ok(())
}

fn start_rebase(new_base_branch: String) -> Result<(), String> {
    // Lógica para iniciar o rebase
    println!("Iniciando rebase na nova base de branch: {}", new_base_branch);
    Ok(())
}