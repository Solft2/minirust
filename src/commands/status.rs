use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{
    objects::{BlobObject, RGitObject, RGitObjectTypes, TreeObject},
    staging::StagingArea,
    utils::find_current_repo,
    Repository,
};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn cmd_status()
{
    if let Err(e) = cmd_status_result()
    {
        eprintln!("Erro ao obter status: {}", e);
    }
}

fn cmd_status_result() -> Result<(), String>
{
    let repo = find_current_repo().ok_or("Não está dentro de um repositório")?;
    let staging_area = StagingArea::new(&repo);

    cmd_status_branch(&repo);
    let has_staged_changes = compare_head_and_index(&repo, &staging_area)?;
    let (has_unstaged_changes, has_untracked_files) = index_worktree_status(&repo, &staging_area)?;

    let worktree_clean = !has_staged_changes && !has_unstaged_changes && !has_untracked_files;
    let nothing_staged_but_worktree_dirty =
        !has_staged_changes && (has_unstaged_changes || has_untracked_files);

    if worktree_clean
    {
        println!("nada para commitar, árvore de trabalho limpa");
    }
    else if nothing_staged_but_worktree_dirty
    {
        println!("\nsem mudanças adicionadas para commitar (utilize o comando \"add\")");
    }

    Ok(())
}

fn cmd_status_branch(repo: &Repository)
{
    if repo.head_detached()
    {
        let head_hash = repo.resolve_head();
        println!("HEAD destacado em {}\n", &head_hash[..7]);
    }
    else
    {
        let head_ref = repo.get_head();
        let branch_name = head_ref
            .strip_prefix("ref: refs/heads/")
            .unwrap_or(&head_ref);
        println!("Na branch {}\n", branch_name);
    }
}

/// Compara HEAD com o índice (staging area)
fn compare_head_and_index(repo: &Repository, staging_area: &StagingArea) -> Result<bool, String>
{
    let head_commit_id = repo.resolve_head();

    if head_commit_id.is_empty()
    {
        return handle_initial_commit(staging_area);
    }

    let head_tree = get_head_tree(repo, &head_commit_id)?;
    let head_dict = tree_to_dict(&head_tree);
    let has_changes = check_staged_changes(staging_area, &head_dict);

    Ok(has_changes)
}

fn handle_initial_commit(staging_area: &StagingArea) -> Result<bool, String>
{
    if staging_area.entries.is_empty()
    {
        return Ok(false);
    }

    println!("Mudanças a serem commitadas:");
    for entry in &staging_area.entries
    {
        println!("  {}novo arquivo: {}{}", GREEN, entry.path.display(), RESET);
    }

    Ok(true)
}

fn check_staged_changes(staging_area: &StagingArea, head_dict: &HashMap<String, String>) -> bool
{
    let mut changes = Vec::new();

    for entry in &staging_area.entries
    {
        let path_str = entry.path.to_string_lossy();

        let action = match head_dict.get(path_str.as_ref())
        {
            Some(head_sha) if head_sha != &entry.object_hash => "modificado",
            None => "novo arquivo",
            _ => continue,
        };

        changes.push((entry.path.display().to_string(), action));
    }

    for path in head_dict.keys()
    {
        let staging_has_file = staging_area
            .entries
            .iter()
            .any(|e| e.path.to_string_lossy() == path.as_str());

        if !staging_has_file
        {
            changes.push((path.clone(), "removido"));
        }
    }

    if !changes.is_empty()
    {
        println!("Mudanças a serem commitadas:");
        for (path, action) in &changes
        {
            println!("  {}{}: {}{}", GREEN, action, path, RESET);
        }
    }

    !changes.is_empty()
}

/// Compara índice com o worktree
fn index_worktree_status(repo: &Repository, staging_area: &StagingArea) -> Result<(bool, bool), String>
{
    let unstaged_files = check_unstaged_changes(repo, staging_area);
    let untracked_files = collect_untracked_files(&repo.worktree, &repo.worktree, staging_area);

    let has_unstaged_changes = !unstaged_files.is_empty();
    let has_untracked_files = !untracked_files.is_empty();

    if has_unstaged_changes
    {
        print_unstaged_changes(&unstaged_files);
    }

    if has_untracked_files
    {
        print_untracked_files(&untracked_files);
    }

    Ok((has_unstaged_changes, has_untracked_files))
}

fn check_unstaged_changes(repo: &Repository, staging_area: &StagingArea) -> Vec<(PathBuf, &'static str)>
{
    staging_area
        .entries
        .iter()
        .filter_map(|entry|
        {
            let absolute_path = repo.worktree.join(&entry.path);

            if !absolute_path.exists()
            {
                return Some((entry.path.clone(), "removido"));
            }

            let blob = BlobObject::from(&absolute_path);
            let new_hash = blob.hash();

            (new_hash != entry.object_hash).then_some((entry.path.clone(), "modificado"))
        })
        .collect()
}

fn print_unstaged_changes(unstaged_files: &[(PathBuf, &str)])
{
    println!("Mudanças não preparadas para commit:");
    for (path, action) in unstaged_files
    {
        println!("  {}{}: {}{}", RED, action, path.display(), RESET);
    }
}

fn print_untracked_files(untracked_files: &[PathBuf])
{
    println!("Arquivos não rastreados:");
    for file in untracked_files
    {
        println!("  {}{}{}", RED, file.display(), RESET);
    }
}

/// Obtém a tree do commit apontado por HEAD
fn get_head_tree(repo: &Repository, head_commit_id: &str) -> Result<TreeObject, String>
{
    let commit = repo
        .get_object(&head_commit_id.to_string())
        .ok_or("Não foi possível obter o commit do HEAD")?;

    let RGitObjectTypes::Commit(c) = commit else
    {
        return Err("HEAD não aponta para um commit".to_string());
    };

    let tree_obj = repo
        .get_object(&c.tree)
        .ok_or("Não foi possível obter a tree do commit")?;

    match tree_obj
    {
        RGitObjectTypes::Tree(t) => Ok(t),
        _ => Err("Objeto referenciado não é uma tree".to_string()),
    }
}

/// Converte uma tree em um dicionário flat de caminhos para hashes
fn tree_to_dict(tree: &TreeObject) -> HashMap<String, String>
{
    let mut result = HashMap::new();
    tree_to_dict_recursive(tree, "", &mut result);
    result
}

fn tree_to_dict_recursive(tree: &TreeObject, prefix: &str, result: &mut HashMap<String, String>)
{
    for child in &tree.children
    {
        let full_path = if prefix.is_empty()
        {
            child.name.clone()
        }
        else
        {
            format!("{}/{}", prefix, child.name)
        };

        result.insert(full_path, child.object_id.clone());
    }
}

/// Coleta todos os arquivos não rastreados no worktree
fn collect_untracked_files(
    current_dir: &Path,
    worktree: &Path,
    staging_area: &StagingArea,
) -> Vec<PathBuf>
{
    let mut untracked = Vec::new();
    collect_untracked_files_recursive(current_dir, worktree, staging_area, &mut untracked);
    untracked
}

fn collect_untracked_files_recursive(
    current_dir: &Path,
    worktree: &Path,
    staging_area: &StagingArea,
    untracked: &mut Vec<PathBuf>,
)
{
    if !current_dir.is_dir()
    {
        return;
    }
    if current_dir
        .file_name()
        .is_some_and(|name| name == Repository::MINIGITDIR)
    {
        return;
    }

    let Ok(entries) = std::fs::read_dir(current_dir) else
    {
        return;
    };

    for entry in entries.flatten()
    {
        let path = entry.path();

        if path.is_dir()
        {
            collect_untracked_files_recursive(&path, worktree, staging_area, untracked);
        }
        else if path.is_file()
        {
            if let Ok(relative_path) = path.strip_prefix(worktree)
            {
                let is_tracked = staging_area
                    .entries
                    .iter()
                    .any(|e| e.path == relative_path);

                if !is_tracked
                {
                    untracked.push(relative_path.to_path_buf());
                }
            }
        }
    }
}
