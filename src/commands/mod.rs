pub mod init;
pub mod hash_rust;
pub mod cat_file;
pub mod tree_rust;
pub mod branch;
pub mod commit;
pub mod log;
pub mod merge;
pub mod add;
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
    Commit,
    Merge,
    Add {
        file: String
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
        Commit => commit::cmd_commit(),
        Merge => merge::cmd_merge(),
        Add { file } => add::cmd_add(&file),
        Commands::HashRust { write, file } => hash_rust::cmd_hash_object(&file, write),
        CatFile { hash } => cat_file::cmd_cat_file(&hash),
        WriteTree => tree_rust::cmd_write_tree(),
    }
}
