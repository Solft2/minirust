use clap::ValueEnum;

use crate::commands::checkout::instanciate_commit;
use crate::{Repository, staging};
use crate::checks::{ensure_no_detached_head, ensure_no_merge_in_progress, ensure_no_rebase_in_progress};
use crate::objects::{CommitObject, RGitObjectTypes};
use crate::staging::staging_area_from_commit;
use crate::utils::{find_current_repo};
use std::fs;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum ResetTypes {
    Soft,
    Mixed,
    Hard,
}

pub fn cmd_reset(mode: ResetTypes, commit_reference: &String) {
    match reset_command_result(mode, commit_reference) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub fn reset_command_result(mode: ResetTypes, commit_reference: &String) -> Result<(), String> {
    let mut repo = find_current_repo().ok_or("Não é um repositório minigit")?;
    ensure_no_detached_head(&repo)?;
    ensure_no_rebase_in_progress(&repo)?;
    ensure_no_merge_in_progress(&repo)?;

    reset(&mut repo, commit_reference, mode)
}

pub fn reset(repo: &mut Repository, commit_reference: &String, mode: ResetTypes) -> Result<(), String> {
    if !repo.reference_exists(commit_reference) {
        return Err("Referência para commit inválida. Deve ser um hash ou nome de branch existente".to_string());
    }

    let commit_hash = repo.resolve_reference(commit_reference);

    // let RGitObjectTypes::Commit(commit_object) = repo
    //     .get_object(&commit_hash)
    //     .ok_or("Commit não encontrado no repositório")?
    //     else {
    //         panic!("Referência não é um commit");
    //     };
    repo.update_head(&commit_hash.to_string());

    match mode {
        ResetTypes::Soft => {
            println!("Soft reset feito para {}", commit_hash);
        }
        ResetTypes::Mixed => {
            let index_path = repo.get_repository_path(&["index"]);

            if commit_hash.is_empty() {
                fs::write(index_path, "").unwrap();
                return Ok(());
            }

            let commit_object = get_commit_object(&repo, &commit_hash);
            let staging_area = staging_area_from_commit(&repo, &commit_object);
            staging::rewrite_index(repo, &staging_area);

            println!("Mixed reset feito para {}", commit_hash);
        }
        ResetTypes::Hard => {
            let index_path = repo.get_repository_path(&["index"]);
            
            if commit_hash.is_empty() {
                fs::write(index_path, "").unwrap();
                repo.clear_worktree();
                return Ok(());
            }

            let commit_object = get_commit_object(&repo, &commit_hash);
            let staging_area = staging_area_from_commit(&repo, &commit_object);
            staging::rewrite_index(repo, &staging_area);
            repo.clear_worktree();
            instanciate_commit(commit_object, repo);

            println!("Hard reset feito para {}", commit_hash);
        }
    }

    Ok(())
}

fn get_commit_object(repo: &Repository, commit_hash: &String) -> CommitObject {
    let RGitObjectTypes::Commit(commit_object) = repo
        .get_object(commit_hash)
        .unwrap()
        else {
            panic!("Referência não é um commit");
        };

    commit_object
}
