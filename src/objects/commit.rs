use crate::{objects::RGitObject};

pub struct CommitObject {
    pub tree: String,
    pub author: String,
    pub message: String,
    pub timestamp: u64,
    pub parent: Vec<String>
}

impl CommitObject {
    pub fn new(content_bytes: Vec<u8>) -> Self {
        let content_str = String::from_utf8(content_bytes).expect("Commit deve ser um arquivo UTF8 válido");
        
        let (tree, remainder) = Self::read_value(&content_str);
        let (author, remainder) = Self::read_value(remainder);
        let (message, remainder) = Self::read_value(remainder);
        let (timestamp_str, mut remainder) = Self::read_value(remainder);

        let timestamp: u64 = timestamp_str.parse().expect("Timestamp deve ser um número válido");
        let mut parent: Vec<String> = Vec::new();

        while !remainder.is_empty() {
            let (parent_commit, new_remainder) = Self::read_value(remainder);
            parent.push(parent_commit);
            remainder = new_remainder;
        }

        Self { tree, author, message, timestamp, parent }
    }

    /// Lê um valor do conteúdo do commit, retornando o valor lido e o restante do conteúdo
    /// 
    /// Ignora-se a primeira palavra (a chave), retornando apenas o valor
    /// O valor pode conter múltiplas linhas, desde que cada linha subsequente comece com um espaço
    fn read_value(content: &str) -> (String, &str) {
        let parts: Vec<&str> = content.splitn(2, ' ').collect();
        let mut remainder = parts[1];

        let new_line = remainder.find('\n').unwrap_or(remainder.len());
        let mut value = remainder[..new_line].to_string();
        remainder = &remainder[new_line+1..];

        while remainder.starts_with(' ') {
            let new_line = remainder.find('\n').unwrap_or(remainder.len());
            value.push('\n');
            value.push_str(&remainder[..new_line]);
            remainder = &remainder[new_line+1..];
        }

        (value, remainder)
    }
}

impl RGitObject for CommitObject {
    /// Ordem precisa ser a mesma da entrada (tree, author, message, timestamp, parent...)
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        let message = self.message.replace("\n", "\n ");
        let parent = self.parent.iter()
            .map(|p| p.replace("\n", "\n "))
            .collect::<Vec<String>>();

        result.extend_from_slice(format!("tree {}\n", self.tree).as_bytes());
        result.extend_from_slice(format!("author {}\n", self.author).as_bytes());
        result.extend_from_slice(format!("message {}\n", message).as_bytes());
        result.extend_from_slice(format!("timestamp {}\n", self.timestamp).as_bytes());
        for parent in &parent {
            result.extend_from_slice(format!("parent {}\n", parent).as_bytes());
        }

        result
    }

    #[allow(unused_variables)]
    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        panic!("Não implementado");
    }

    fn object_type(&self) -> &'static str {
        "commit"
    }
}