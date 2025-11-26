use std::fs;
use std::io::Read;
use flate2::read::ZlibDecoder;
use crate::Repository;

/// TODO: Atualizar
pub fn cmd_cat_file(hash: &str){
    let repo = Repository::new(&std::env::current_dir().unwrap());

    let(dir,file) = hash.split_at(2);
    let path = repo.get_repository_path(&["objects", dir, file]);

    if !path.exists(){
        println!("Não foi possível encontrar o objeto: {}", hash);
        return;
    }

    let compressed = fs::read(path).expect("Não foi possível ler o objeto");
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();

    decoder.read_to_end(&mut decompressed).expect("Não foi possível descomprimir!");

    let nul_pos = decompressed.iter().position(|&b| b == 0).expect("Formato inválido do objeto não identificado");
    let header = &decompressed[..nul_pos];
    let content = &decompressed[nul_pos + 1..];

    let header_str = String::from_utf8_lossy(header);
    let object_type = header_str.split_whitespace().next().unwrap_or("unknown");

    println!("Tipo de objeto: {}", object_type);
    println!("Conteúdo: \n{}", String::from_utf8_lossy(content));
}