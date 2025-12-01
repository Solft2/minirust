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

use crate::commands::reset::ResetTypes;


#[derive(Parser)]
#[command(version)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Inicializa um novo repositório Minigit
    Init,
    /// Clona um repositório Minigit existente no caminho especificado
    Clone {
        /// Caminho do repositório a ser clonado
        repository_path: String,
        /// Caminho de destino para o repositório clonado
        destination_path: String,
    },
    Diff,
    /// Exibe o histórico a partir do commit atual
    Log,
    /// Cria uma nova branch ou deleta uma branch existente
    Branch {
        /// Deleta a branch especificada
        #[arg(short, long)]
        delete: bool,
        /// Nome da branch
        branch_name: String
    },
    /// Realiza o merge da branch especificada na branch atual
    Merge {
        branch_name: Option<String>,
        #[arg(long)]
        abort: bool,
        #[arg(long)]
        continue_: bool
    },
    /// Muda a base da branch atual para a branch especificada
    Rebase {
        /// Continua um rebase interrompido por conflitos
        #[arg(long, conflicts_with_all(["abort", "new_base_branch"]))]
        continue_: bool,
        /// Aborta um rebase interrompido por conflitos
        #[arg(long, conflicts_with_all(["continue_", "new_base_branch"]))]
        abort: bool,
        /// Nome da nova branch base para o rebase
        new_base_branch: Option<String>
    },
    /// Adiciona arquivos ao índice para o próximo commit
    Add {
        /// Lista de arquivos a serem adicionados
        files: Vec<String>
    },
    /// Muda para o commit especificado
    Checkout {
        /// ID do commit para o qual mudar
        commit_reference: String
    },
    /// Cria um novo commit com as mudanças no índice
    Commit {
        /// Mensagem do commit
        message: String
    },
    /// Lista o conteúdo de uma árvore especificada
    LsTree {
        /// Hash da árvore a ser listada
        tree_id: String
    },
    /// Configura uma chave e valor no arquivo de configuração do Minigit
    Config {
        /// Chave de configuração
        key: String,
        /// Valor de configuração
        value: String,
    },
    /// Gera o hash SHA-1 de um arquivo e opcionalmente o armazena no repositório
    HashObject {
        /// Escreve o objeto no repositório
        #[arg(short, long)]
        write: bool,
        /// Caminho do arquivo
        file: String,
    },
    /// Exibe o conteúdo de um objeto armazenado no repositório
    CatFile{ 
        /// Hash SHA-1 do objeto a ser exibido
        hash: String 
    },
    /// Muda o HEAD para apontar para um commit específico
    Reset {
        /// Tipo de reset a ser realizado
        #[arg(long)]
        mode: ResetTypes,
        /// 
        commit_reference: String
    },
}


pub fn cli_main() {
    use Commands::*;
    let args = CliArgs::parse();

    match args.command {
        Init => init::cmd_init(),
        Clone { 
            repository_path, 
            destination_path 
        } => clone::cmd_clone(&repository_path, &destination_path),
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
        Checkout { commit_reference } => checkout::cmd_checkout(&commit_reference),
        Commit { message } => commit::cmd_commit(message),
        LsTree { tree_id } => ls_tree::cmd_ls_tree(tree_id),
        Config { key, value } => config::cmd_config(key, value),
        HashObject { write, file } => hash_rust::cmd_hash_object(&file, write),
        CatFile { hash } => cat_file::cmd_cat_file(&hash),
        Reset { mode, commit_reference } => reset::cmd_reset(mode, &commit_reference),
    }
}
