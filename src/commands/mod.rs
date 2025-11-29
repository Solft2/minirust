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
    Log,
    Branch,
    Merge,
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
}

pub fn cli_main() {
    use Commands::*;
    let args = CliArgs::parse();

    match args.command {
        Init => init::cmd_init(),
        Log => log::cmd_log(),
        Branch => branch::cmd_branch(),
        Merge => merge::cmd_merge(),
        Add { files } => add::cmd_add(files),
        Checkout { commit_id } => checkout::cmd_checkout(&commit_id),
        Commit { message } => commit::cmd_commit(message),
        LsTree { tree_id } => ls_tree::cmd_ls_tree(tree_id),
        Config { key, value } => config::cmd_config(key, value),
        HashRust { write, file } => hash_rust::cmd_hash_object(&file, write),
        CatFile { hash } => cat_file::cmd_cat_file(&hash),
        WriteTree => tree_rust::cmd_write_tree(),
    }
}
