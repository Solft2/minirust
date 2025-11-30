use crate::{Repository, checks::{ensure_no_merge_in_progress, ensure_no_rebase_in_progress}, utils::{find_current_repo, is_valid_sha1}};

pub fn cmd_branch(branch_name: String, delete: bool) {
    match cmd_branch_result(branch_name, delete) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_branch_result(branch_name: String, delete: bool) -> Result<(), String> {
    if branch_name == Repository::HEAD || is_valid_sha1(&branch_name) || branch_name.ends_with("index") {
        return Err("Nome de branch não pode ser 'HEAD', um hash SHA-1 válido ou terminar com 'index'".to_string());
    }

    let mut repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    ensure_no_merge_in_progress(&repo)?;
    ensure_no_rebase_in_progress(&repo)?;

    if delete {
        delete_branch(&branch_name, &mut repo)?;
        return Ok(());
    }

    create_branch(&branch_name, &mut repo)?;

    Ok(())
}

fn delete_branch(branch_name: &String, repo: &mut Repository) -> Result<(), String> {
    let branch_index = repo.refs_heads_path.join(branch_name).join(Repository::INDEX);

    if !branch_index.exists() {
        return Err("Branch não existe".to_string());
    }

    let head_ref = repo.get_head();
    let head_branch_index = repo.minigitdir.join(head_ref).join(Repository::INDEX);

    if head_branch_index == branch_index {
        return Err("Não é possível deletar a branch atualmente ativa".to_string());
    }

    std::fs::remove_file(branch_index).map_err(|_| "Erro ao deletar a branch".to_string())?;
    Ok(())
}

fn create_branch(branch_name: &String, repo: &mut Repository) -> Result<(), String> {
    let branch_index = repo.refs_heads_path.join(branch_name).join(Repository::INDEX);

    if branch_index.exists() {
        return Err("Branch já existe".to_string());
    }

    let head_commit = repo.resolve_head();

    std::fs::create_dir_all(branch_index.parent().unwrap()).unwrap();
    std::fs::write(branch_index, head_commit).unwrap();

    Ok(())
}