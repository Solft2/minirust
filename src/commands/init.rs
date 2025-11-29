use std::fs::File;
use std::io::Write;
use crate::Repository;
use crate::utils::find_repo;

pub fn cmd_init() {
    let current = std::env::current_dir().expect("Deveria acessar o diretório atual");
    let repo = find_repo(&current);

    match repo {
        Some(_repository) => {
            println!("Este diretório já faz parte de um repostitório. Ainda não suportamos repositórios dentro de outros repositórios");
            return;
        }
        None => {}
    }

    let mut repo = Repository::new(&current);
    create_repo(&mut repo);
    println!("Repositório Minigit inicializado em {:?}", repo.worktree);
}

fn create_repo(repo: &mut Repository) {
    repo.create_repository_dir(&[]);
    repo.create_repository_dir(&["objects"]);
    repo.create_repository_dir(&["refs"]);
    repo.create_repository_dir(&["refs", "tags"]);
    repo.create_repository_dir(&["refs", "heads"]);
    repo.create_repository_file(&["index"]);
    repo.create_repository_file(&["refs", "heads", "master"]);
    repo.create_repository_file(&["config"]);
    
    let mut description_file = repo.create_repository_file(&["description"]);
    let mut head_file = repo.create_repository_file(&["HEAD"]);

    write_file(&mut description_file, "Repositório sem nome. Edite este arquivo para nomear.\n");
    write_file(&mut head_file, "ref: refs/heads/master");
}

fn write_file(file: &mut File, content: &str) {
    file.write_all(content.as_bytes()).expect("Erro escrevendo arquivo!");
}
