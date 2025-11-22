use std::io::Write;
use std::fs;
use hex;

use crate::commands::hash_rust::{make_blob, sha1sum, write_object};
use crate::Repository;

pub struct Trees {
    pub mode: String,
    pub name: String,
    pub hash: String,
}

pub fn serialize(inputs: &[Trees]) -> Vec<u8> {
    let mut result = Vec::new();

    for entry in inputs {
        result.write_all(format!("{} {}\0", entry.mode, entry.name).as_bytes())
            .unwrap();

        let hash_bytes = hex::decode(&entry.hash).unwrap();
        result.write_all(&hash_bytes).unwrap();
    }
    result
}

pub fn create_tree(repo: &Repository, inputs: &[Trees]) -> String {
    let body = serialize(inputs);

    let header = format!("tree {}\0", body.len());
    let mut full = header.into_bytes();
    full.extend(body);

    let hash = sha1sum(&full);
    write_object(repo, &hash, &full);
    hash
}

pub fn cmd_write_tree() {
    let repo = Repository::new(&std::env::current_dir().unwrap());

    let mut inputs = Vec::new();

    for input in fs::read_dir(".").unwrap() {
        let input = input.unwrap();
        let path = input.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        if name == ".git" {
            continue;
        }

        if path.is_file() {
            let byte = fs::read(&path).unwrap();
            let blob_bytes = make_blob(&byte);
            let hash = sha1sum(&blob_bytes);
            write_object(&repo, &hash, &blob_bytes);

            inputs.push(Trees {
                mode: "100644".to_string(),
                name,
                hash,
            });
        }
    }

    let hash = create_tree(&repo, &inputs);
    println!("{}", hash);
}
