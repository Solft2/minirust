use crate::{Repository, commands::checkout, objects::RGitObjectTypes, utils::find_current_repo};

pub fn cmd_merge(branch_name: &String) {
    match execute_merge(branch_name) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn execute_merge(branch_name: &String) -> Result<(), String> {
    let mut repo = find_current_repo().ok_or("Diretório não está dentro um repositório minigit")?;

    // 
    let current_head_hash = repo.resolve_head();
    if current_head_hash.is_empty() {
        return Err("Nada para fazer merge, repositório vazio.".to_string());
    }

    let target_branch_path = repo.minigitdir.join("refs").join("heads").join(&branch_name).join("index");
    if !target_branch_path.exists() {
        return Err(format!("Branch {} não existe.", branch_name));
    }

    let target_hash = std::fs::read_to_string(target_branch_path)
        .map_err(|_| "Não foi possível ler a referência da branch alvo.")?
        .trim()
        .to_string();

    if current_head_hash == target_hash {
        println!("Branch já atualizada.");
        return Ok(());
    }

    // Tentar realizar o merge fast-forward
    if is_ancestor(&repo, &current_head_hash, &target_hash) {
        repo.update_curr_branch(&target_hash);
        repo.clear_worktree();

        let target_object = repo.get_object(&target_hash).ok_or("Objeto da branch alvo não encontrado.")?;

        checkout::instanciate_commit(target_object, &mut repo);

        println!("Merge concluído. Branch {} atualizada para {}.", repo.get_head(), target_hash);
    } else {
        return Err("Erro: não é possível fazer Fast-Forward.".to_string());
    }

    return Ok(());
}

/// Verifica se 'possible_ancestor' está no histórico de 'descendant'
fn is_ancestor(repo: &Repository, possible_ancestor: &String, descendant: &String) -> bool {
    let mut fila = vec![descendant.clone()];

    while let Some(current_hash) = fila.pop() {
        if &current_hash == possible_ancestor {
            return true;
        }

        if let Some(RGitObjectTypes::Commit(commit)) = repo.get_object(&current_hash) {
            for parent in commit.parent {
                fila.push(parent);
            }
        }
    }

    return false;
}
