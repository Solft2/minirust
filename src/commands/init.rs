use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use crate::Repository;



pub fn cmd_init() {
    //Diretorio atual
    let current = std::env::current_dir().expect("Não foi possível acessar o diretório");
    //diretorio atual/.git
    let repo = Repository::new(&current);

    create_repo(&repo);
    println!("Repositório Git inicializado em {:?}", repo.worktree);
}

fn create_repo(repo: &Repository) {

    //verefica se já não existe um .git
    if repo.gitdir.exists() && repo.gitdir.read_dir().unwrap().next().is_some() {
        panic!("Já existe um repositório Git aqui!");
    }
    //cria tudo isso dentro da .git
    repo.repository_dir(&[], true); // propria .git
    repo.repository_dir(&["objects"], true);
    repo.repository_dir(&["refs"], true);
    repo.repository_dir(&["refs", "heads"], true);
    repo.repository_dir(&["refs", "tags"], true);
    repo.repository_file(&["index"], true);

    write_file(repo.repository_file(&["description"], true),
        "Repositório sem nome. Edite este arquivo para nomear.\n");

    write_file(repo.repository_file(&["HEAD"], true),
        "ref: refs/heads/master\n");

    write_file(repo.repository_file(&["config"], true),
        "[core]\n\
         repositoryformatversion = 0\n\
         filemode = false\n\
         bare = false\n");
}

fn write_file(path: PathBuf, content: &str) {
    let mut file = File::create(path).expect("Erro criando arquivo!");
    file.write_all(content.as_bytes()).expect("Erro escrevendo arquivo!");
}
