pub mod init;
pub mod hash_rust;
pub mod cat_file;
pub mod tree_rust;
pub mod branch;
pub mod commit;
pub mod log;
pub mod merge;
pub mod add;
pub mod checkout;
pub mod ls_tree;
pub mod config;
pub mod clone;
pub mod reset;
pub mod rebase;
pub mod diff;

use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Clone {
        repository_path: String,
        destination_path: String,
    },
    Diff,
    Log,
    Branch {
        #[arg(short, long)]
        delete: bool,
        branch_name: String
    },
    Merge {
        branch_name: Option<String>,
        #[arg(long)]
        abort: bool,
        #[arg(long)]
        continue_: bool
    },
    Rebase {
        #[arg(long, conflicts_with_all(["abort", "new_base_branch"]))]
        continue_: bool,

        #[arg(long, conflicts_with_all(["continue_", "new_base_branch"]))]
        abort: bool,

        new_base_branch: Option<String>
    },
    Add {
        files: Vec<String>
    },
    Checkout {
        commit_id: String
    },
    Commit {
        message: String
    },
    LsTree {
        tree_id: String
    },
    Config {
        key: String,
        value: String,
    },
    HashRust {
        #[arg(short, long)]
        write: bool,
        file: String,
    },
    CatFile{hash: String},
    WriteTree,
    Reset {
    files: Vec<String>
},
}

pub fn cli_main() {
    use Commands::*;
    let args = CliArgs::parse();

    match args.command {
        Init => init::cmd_init(),
        Clone { repository_path, destination_path } => clone::cmd_clone(&repository_path, &destination_path),
        Diff => diff::cmd_diff(),
        Log => log::cmd_log(),
        Branch { branch_name, delete } => branch::cmd_branch(branch_name, delete),
        Merge {branch_name, abort, continue_ } => {
            if continue_ || abort {
                merge::cmd_merge(None, abort, continue_);
            } else {
                if branch_name.is_none() {
                    println!("Erro: Forneça o nome da branch, --abort ou --continue");
                    return;
                }
                merge::cmd_merge(branch_name.as_ref(), abort, continue_);
            }

        },
        Rebase { continue_, abort, new_base_branch } => {rebase::cmd_rebase(continue_, abort, new_base_branch)},
        Add { files } => add::cmd_add(files),
        Checkout { commit_id } => checkout::cmd_checkout(&commit_id),
        Commit { message } => commit::cmd_commit(message),
        LsTree { tree_id } => ls_tree::cmd_ls_tree(tree_id),
        Config { key, value } => config::cmd_config(key, value),
        HashRust { write, file } => hash_rust::cmd_hash_object(&file, write),
        CatFile { hash } => cat_file::cmd_cat_file(&hash),
        WriteTree => tree_rust::cmd_write_tree(),
        Reset { files } => reset::cmd_reset(files),
    }
}
