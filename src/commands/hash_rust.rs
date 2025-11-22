use std::fs;
use std::io::Write;
use sha1::{Digest, Sha1};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use crate::Repository;


pub fn cmd_hash_object(path: &str, write:bool){
    // cria uma instancia apontando para .git atual
    let repo = Repository::new(&std::env::current_dir().unwrap());
    //Lê o conteudo do arquivo em bytes
    let byte = fs::read(path).expect("Não foi possível ler o arquivo!");
    //cria o blob no formato padrão
    let blob = make_blob(&byte);
    //Calcula o hash sha-1 do arquivo blolb criado anteriormente
    let hash = sha1sum(&blob);

    if write{
        write_object(&repo,&hash,&blob);
    }
    println!("{}",hash);
}
// blolb tamanho\0 conteudo do arquivo
pub fn make_blob(byte: &[u8]) -> Vec<u8>{
    let header = format!("blob {}\0", byte.len());
    let mut blob = header.into_bytes();
    blob.extend_from_slice(byte);
    blob
}

pub fn sha1sum(byte: &[u8]) -> String{
    let mut hasher = Sha1::new();
    hasher.update(byte);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn write_object(repo: &Repository, hash: &str, byte: &[u8]){
    // divide o blob em 2, os dois primeiros caracteres e o resto
    let(dir, file) = hash.split_at(2);
    // cria o diretorio como .git/objects/dir = os dois primeiros caracteres/file = restante do arquivo
    let path = repo.repository_dir(&["objects",dir],true).unwrap().join(file);
    // se já existe não salva novamente
    if path.exists(){
        return;
    }
    //compacta
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(byte).unwrap();
    let compressed = encoder.finish().unwrap();
    //Escreve o blolb compactado no disco
    fs::write(path, compressed).unwrap();
}