use crate::utils::find_current_repo;

pub fn cmd_config(key: String, value: String) {
    match cmd_config_result(key.clone(), value.clone()) {
        Ok(_) => {
            println!("Chave '{}' atualizada para '{}' com sucesso.", &key, &value);
        }
        Err(e) => println!("Erro ao atualizar configuração: {}", e),
    }
}

fn cmd_config_result(key: String, value: String) -> Result<(), String> {
    let mut repo = find_current_repo().ok_or("Não é um repositório minigit")?;
    repo.update_config(key, value);
    Ok(())
}