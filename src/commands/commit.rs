use crate::{checks::{ensure_no_detached_head, ensure_no_merge_in_progress, ensure_no_rebase_in_progress}, objects::create_commit_object_from_index, utils::{find_current_repo, merge_rebase::finish}};

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

    ensure_no_detached_head(&repo)?;
    ensure_no_merge_in_progress(&repo)?;
    ensure_no_rebase_in_progress(&repo)?;

    let commit_hash = create_commit_object_from_index(&mut repo, message);

    repo.update_curr_branch(&commit_hash);

    if repo.merge_head_path.exists() {
        finish(&mut repo, false);
        println!("Estado de merge finalizado e limpo");
    }

    Ok(commit_hash)
}
