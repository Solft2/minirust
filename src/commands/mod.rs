pub mod init;
pub mod hash_rust;
pub mod cat_file;
pub mod tree_rust;
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
        Commands::HashRust { write, file } => hash_rust::cmd_hash_object(&file, write),
        CatFile { hash } => cat_file::cmd_cat_file(&hash),
        WriteTree => tree_rust::cmd_write_tree(),
    }
}
