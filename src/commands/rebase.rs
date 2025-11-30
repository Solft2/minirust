use core::panic;
use std::{collections::{HashMap, HashSet}, path::PathBuf, str::FromStr};

use crate::{Repository, checks::{ensure_no_detached_head, ensure_no_merge_in_progress, ensure_no_non_staged_files, ensure_no_rebase_in_progress, ensure_no_uncommited_changes, ensure_rebase_in_progress}, objects::{BlobObject, CommitObject, RGitObject, RGitObjectTypes, create_commit_object_from_index, create_tree_object_from_staging_tree, get_commit_tree_as_map, get_tree_as_map, instanciate_tree_files}, staging::StagingTree, utils::{find_current_repo, merge_rebase}};

pub fn cmd_rebase(continue_: bool, abort: bool, new_base_reference: Option<String>) {
    match cmd_rebase_result(continue_, abort, new_base_reference) {
        Ok(_) => {},
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn cmd_rebase_result(continue_: bool, abort: bool, new_base_reference: Option<String>) -> Result<(), String> {
    let mut repo = find_current_repo()
        .ok_or("Diretório não está dentro um repositório minigit")?;

    if continue_ {
        continue_rebase(&mut repo)?;
    } else if abort {
        abort_rebase(&mut repo)?;
    } else if let Some(new_base_reference) = new_base_reference.clone() {
        initialize_rebase_command(&mut repo, new_base_reference)?;
    } else {
        return Err("Você deve fornecer nova base de branch para rebase.".to_string());
    }

    merge_rebase::finish(&mut repo, true);
    Ok(())
}

fn abort_rebase(repo: &mut Repository) -> Result<(), String> {
    ensure_rebase_in_progress(repo)?;
    merge_rebase::abort(repo, true);
    
    Ok(())
}

/// Inicia o processo de rebase no repositório atual
fn initialize_rebase_command(repo: &mut Repository, new_base_reference: String) -> Result<(), String> {
    ensure_no_uncommited_changes(repo)?;
    ensure_no_detached_head(repo)?;
    ensure_no_merge_in_progress(repo)?;
    ensure_no_rebase_in_progress(repo)?;

    let exists = repo.reference_exists(&new_base_reference);
    if !exists {
        return Err("A referência fornecida não existe.".to_string());
    }

    merge_rebase::start(repo, true);
    start_rebase(new_base_reference, repo)?;

    Ok(())
}

fn continue_rebase(repo: &mut Repository) -> Result<(), String> {
    ensure_no_non_staged_files(repo)?;
    ensure_rebase_in_progress(repo)?;

    let commits_to_apply = merge_rebase::get_state(repo, true).iter()
        .map(|hash| {
            let RGitObjectTypes::Commit(commit) = repo.get_object(hash).unwrap() else {
                panic!("Objeto referenciado por merge_head não é um commit");
            };
            commit
        })
        .collect::<Vec<CommitObject>>();

    let original_commit = commits_to_apply.first().unwrap();

    let commit_hash = create_commit_object_from_index(repo, original_commit.message.clone());
    repo.update_curr_branch(&commit_hash);

    apply_commits(repo, commits_to_apply[1..].to_vec(), repo.get_head())?;
    Ok(())
}

fn start_rebase(new_base_reference: String, repo: &mut Repository) -> Result<(), String> {
    let current_branch_ref_path = repo.get_head();
    let current_branch_head = repo.resolve_head();
    let new_base_head = repo.resolve_reference(&new_base_reference);

    let curr_branch_history = repo.get_commit_history_from_commit(&current_branch_head);
    let new_base_history = repo.get_commit_history_from_commit(&new_base_head);
    let base_commit = base_commit(&curr_branch_history, &new_base_history);

    let commits_to_apply: Vec<CommitObject> = if new_base_head.is_empty() {
        curr_branch_history
    } else if current_branch_head.is_empty() {
        Vec::new()
    } else {
        commits_to_apply(curr_branch_history, &base_commit)
    };

    if commits_to_apply.is_empty() {
        println!("Nada a aplicar, os históricos já estão alinhados.");
        return Ok(());
    }

    repo.update_branch_ref(&current_branch_ref_path, &new_base_head);

    apply_commits(repo, commits_to_apply, current_branch_ref_path)?;

    let current_base_head = repo.resolve_head();
    let RGitObjectTypes::Commit(current_base_head_commit) = repo
        .get_object(&current_base_head)
        .unwrap() else {
            panic!("Objeto referenciado por current_base_head não é um commit");
        };
    let RGitObjectTypes::Tree(current_base_head_commit_tree) = repo
        .get_object(&current_base_head_commit.tree)
        .unwrap() else {
            panic!("Objeto referenciado por current_base_head_commit.tree não é uma tree");
        };

    instanciate_tree_files(repo, &current_base_head_commit_tree);
    println!("Rebase concluído. HEAD atual: {}", current_base_head);
    Ok(())
}

fn apply_commits(repo: &mut Repository, commits_to_apply: Vec<CommitObject>, current_branch_ref_path: String) -> Result<(), String> {
    for (index, commit) in commits_to_apply.iter().enumerate() {
        let current_base_head = repo.resolve_head();
        let RGitObjectTypes::Commit(current_base_head_commit) = repo
            .get_object(&current_base_head)
            .unwrap() else {
                panic!("Objeto referenciado por current_base_head não é um commit");
            };

        let (merge_tree_id, conflicts) = create_merge_tree(repo, &commit, &current_base_head_commit);
        
        if conflicts.is_empty() {
            let commit_id = create_rebase_commit(repo, &commit, current_base_head.clone(), merge_tree_id);
            repo.update_branch_ref(&current_branch_ref_path, &commit_id);
        } else {
            let remaining_commits: Vec<CommitObject> = commits_to_apply[index..].to_vec();
            let RGitObjectTypes::Tree(merge_tree_obj) = repo
                .get_object(&merge_tree_id)
                .unwrap() else {
                    panic!("Objeto referenciado por merge_tree_id não é uma tree");
                };
            let merge_tree_files = get_tree_as_map(repo, &merge_tree_obj);
            let non_conflict_files = get_non_conflict_files(&merge_tree_files, &conflicts);

            instanciate_tree_files(repo, &merge_tree_obj);
            repo.add_files(non_conflict_files);
            interrupt_rebase(repo, remaining_commits);

            let conflict_messages = get_conflict_files_messages(&conflicts);
            return Err(format!("Conflitos encontrados durante o rebase nos arquivos:\n{}", conflict_messages));
        }
    }

    Ok(())
}

fn create_rebase_commit(
    repo: &mut Repository, 
    original_commit: &CommitObject,
    current_branch_head: String,
    merge_tree_id: String, 
) -> String {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();

    let rebase_commit = CommitObject {
        tree: merge_tree_id,
        message: original_commit.message.clone(),
        author: original_commit.author.clone(),
        timestamp: now,
        parent: vec![current_branch_head],
    };

    repo.create_object(&rebase_commit)
}

fn interrupt_rebase(repo: &mut Repository, not_applied_commits: Vec<CommitObject>) {
    let mut rebase_head_content = String::new();
    for commit in not_applied_commits {
        rebase_head_content.push_str(&commit.hash());
        rebase_head_content.push('\n');
    }

    std::fs::write(&repo.rebase_head_path, rebase_head_content).unwrap();
}

/// Retorna o commit base comum entre os dois históricos, se existir
fn base_commit(history_a: &Vec<CommitObject>, history_b: &Vec<CommitObject>) -> Option<CommitObject> {
    for commit_a in history_a {
        for commit_b in history_b {
            if commit_a.hash() == commit_b.hash() {
                return Some(commit_a.clone());
            }
        }
    }

    None
}

/// Retorna os commits que precisam ser aplicados na nova base.
/// Eles estão ordenados do mais antigo para o mais recente.
fn commits_to_apply(curr_branch_history: Vec<CommitObject>, base_commit: &Option<CommitObject>) -> Vec<CommitObject> {
    match base_commit {
        None => return curr_branch_history,
        Some(base_commit) => {
            let mut commits = Vec::new();
            for commit in curr_branch_history {
                if commit.hash() == base_commit.hash() {
                    break;
                }
                commits.push(commit);
            }
            commits.reverse();
            commits
        },
    }
}

/// Cria uma tree no repositório representando o merge commit_a e commit_b.
/// Retorna o hash da nova tree criada caso não haja conflitos e a lista dos arquivos que deram conflito.
/// Esta função deve entrar em pânico se ocorrer um erro inesperado.
fn create_merge_tree(repo: &mut Repository, commit_a: &CommitObject, commit_b: &CommitObject) -> (String, HashSet<String>){
    let commit_a_tree: HashMap<String, String> = get_commit_tree_as_map(repo, commit_a);
    let commit_b_tree: HashMap<String, String> = get_commit_tree_as_map(repo, commit_b);
    let mut merge_commit_tree: HashMap<String, String> = commit_b_tree.clone();
    let mut conflicts: HashSet<String> = HashSet::new();

    for (file_path, hash_a) in &commit_a_tree {
        if let Some(hash_b) = commit_b_tree.get(file_path) {
            if hash_a != hash_b {
                conflicts.insert(file_path.clone());
            }
        } else {
            merge_commit_tree.insert(file_path.clone(), hash_a.clone());
        }
    }
    
    let mut merge_staging_tree = StagingTree::Fork(HashMap::new());
    for (file_path, hash_obj) in &merge_commit_tree {
        merge_staging_tree.insert(hash_obj.clone(), PathBuf::from_str(&file_path).unwrap());
    }
    
    if !conflicts.is_empty() {
        for conflicted_file_path in &conflicts {
            let commit_a_blob = repo.get_object(&commit_a_tree[conflicted_file_path]).unwrap();
            let commit_b_blob = repo.get_object(&commit_b_tree[conflicted_file_path]).unwrap();

            let commit_a_blob = match commit_a_blob {
                RGitObjectTypes::Blob(blob) => blob.content,
                _ => panic!("Objeto esperado é um blob"),
            };

            let commit_b_blob = match commit_b_blob {
                RGitObjectTypes::Blob(blob) => blob.content,
                _ => panic!("Objeto esperado é um blob"),
            };

            let blob_id = create_conflict_blob(repo, commit_a_blob, commit_b_blob);
            merge_staging_tree.insert(blob_id, PathBuf::from_str(conflicted_file_path).unwrap());
        }
    }

    let merge_tree_id = create_tree_object_from_staging_tree(&merge_staging_tree, repo);
    (merge_tree_id, conflicts)
}

fn create_conflict_blob(repo: &mut Repository, content_a: Vec<u8>, content_b: Vec<u8>) -> String {
    let mut conflict_content: Vec<u8> = Vec::new();
    conflict_content.extend_from_slice(b"<<<<<<< HEAD\n");
    conflict_content.extend_from_slice(&content_a);
    conflict_content.extend_from_slice(b"\n=======\n");
    conflict_content.extend_from_slice(&content_b);
    conflict_content.extend_from_slice(b"\n>>>>>>>");

    let blob = BlobObject {
        content: conflict_content,
    };

    repo.create_object(&blob)
}

fn get_non_conflict_files(merge_tree_files: &HashMap<String, String>, conflicts: &HashSet<String>) -> Vec<PathBuf> {
    merge_tree_files
        .keys()
        .filter(|file_path_str| conflicts.get(*file_path_str).is_none())
        .cloned()
        .map(|file_path_str| PathBuf::from_str(&file_path_str).unwrap())
        .collect()
}

fn get_conflict_files_messages(conflicts: &HashSet<String>) -> String {
    conflicts.iter()
        .map(|file_path| format!("- {}", file_path))
        .collect::<Vec<String>>()
        .join("\n")
}
