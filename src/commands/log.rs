use chrono::{DateTime, Local, TimeZone};

use crate::{objects::RGitObject, utils::find_current_repo};

pub fn cmd_log() {
    match cmd_log_result() {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn cmd_log_result() -> Result<(), String> {
    let repo = find_current_repo().ok_or("Não está dentro de um repositório")?;
    let commits = repo.get_commit_history();

    if commits.is_empty() {
        println!("Nenhum commit encontrado no repositório.");
        return Ok(());
    }

    for commit in commits {
        let dt: DateTime<Local> = Local.timestamp_opt(commit.timestamp as i64, 0).unwrap();
        let formatted_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();

        println!("commit {}", commit.hash());
        println!("Autor: {}", commit.author);
        println!("Data:  {}", formatted_date);
        println!();
        println!("\t{}", commit.message);
        println!();
    }
    Ok(())
}