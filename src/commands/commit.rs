use crate::{objects::{create_commit_object_from_index}, utils::find_current_repo};

pub fn cmd_commit(message: String) {
    match cmd_commit_result(message) {
        Ok(hash) => {
            println!("Commit criado com o hash {}", hash);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_commit_result(message: String) -> Result<String, String> {
    let mut repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    if repo.head_detached() {
        return Err("Não é possível criar um commit com o HEAD destacado".to_string());
    }

    let commit_hash = create_commit_object_from_index(&mut repo, message);

    repo.update_curr_branch(&commit_hash);

    Ok(commit_hash)
}
