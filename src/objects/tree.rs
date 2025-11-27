use core::panic;

use crate::objects::RGitObject;

pub struct TreeObject {
    pub children: Vec<TreeObjectChild>
}

pub struct TreeObjectChild {
    pub mode: String,
    pub object_id: String,
    pub path: String
}

impl RGitObject for TreeObject {
    fn serialize(&self) -> Vec<u8> {
        let mut result = String::new();

        for child in &self.children {
            let helper = format!("{} {}\0{}\n", &child.mode, &child.path, &child.object_id);
            result.push_str(&helper);
        }

        result.as_bytes().to_vec()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        self.children = Self::children_from_bytes(object_bytes);
    }
}

impl TreeObject {
    pub fn new(object_bytes: Vec<u8>) -> Self {
        TreeObject { children: Self::children_from_bytes(object_bytes) }
    }

    fn children_from_bytes(object_bytes: Vec<u8>) -> Vec<TreeObjectChild> {
        let mut result: Vec<TreeObjectChild> = Vec::new();
        let mut object_str = str::from_utf8(&object_bytes).expect("O objeto deve ser uma string UTF-8 válida");

        while let Some(new_line) = object_str.find('\n') {
            let record = &object_str[..new_line];
            result.push(Self::parse_child(record));
            object_str = &object_str[new_line+1..];
        }

        result
    }

    fn parse_child(record: &str) -> TreeObjectChild {
        let space = record.find(' ');
        let null = record.find('\0');

        if space.is_none() || null.is_none() {
            panic!("Objeto árvore mal formatado");
        }

        let space = space.unwrap();
        let null = null.unwrap();

        TreeObjectChild { 
            mode: record[..space].to_string(), 
            object_id: record[space+1..null].to_string(), 
            path: record[null+1..].to_string()
        }
    }
}