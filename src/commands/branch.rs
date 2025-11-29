use crate::utils::find_current_repo;

pub fn cmd_branch(branch_name: String, delete: bool) {
    match cmd_branch_result(branch_name, delete) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_branch_result(branch_name: String, delete: bool) -> Result<(), String> {
    let mut repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    if delete {
        repo.delete_branch(&branch_name)?;
        return Ok(());
    }

    repo.create_branch(&branch_name)?;

    Ok(())
}