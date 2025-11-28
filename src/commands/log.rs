use chrono::{DateTime, Local, TimeZone};
use std::process::{Command, Stdio};
use std::io::Write;

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
    let mut output = String::new();

    if commits.is_empty() {
        println!("Nenhum commit encontrado no repositório.");
        return Ok(());
    }

    for commit in commits {
        let dt: DateTime<Local> = Local.timestamp_opt(commit.timestamp as i64, 0).unwrap();
        let formatted_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();

        output.push_str(&format!("commit {}\n", commit.hash()));
        output.push_str(&format!("Autor: {}\n", commit.author));
        output.push_str(&format!("Data:  {}\n", formatted_date));
        output.push_str("\n");
        output.push_str(&format!("\t{}\n", commit.message));
        output.push_str("\n");
    }

    run_with_pager(&output);
    Ok(())
}

/// Executa o comando 'less' para paginar a saída
/// 
/// Depende que o `less` esteja instalado no sistema
/// 
/// ## Arguments
/// * `output` - A string que será passada para o pager
fn run_with_pager(output: &str) {
    let mut child = Command::new("less")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.as_mut().unwrap().write_all(output.as_bytes()).unwrap();
    child.wait().unwrap();
}