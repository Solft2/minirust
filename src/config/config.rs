use std::collections::HashMap;

use crate::utils::files;

pub struct GitConfig {
    dict: HashMap<String, String>,
}

impl GitConfig {
    pub fn new(config_bytes: Vec<u8>) -> Self {
        let config_str = String::from_utf8(config_bytes).expect("Config deve ser um arquivo UTF8 válido");
        let mut content_str = config_str.as_str();
        let mut dict: HashMap<String, String> = HashMap::new();

        while !content_str.is_empty() {
            let (key, value, remainder) = files::read_value(content_str);
            dict.insert(key, value);
            content_str = remainder;
        }
        
        GitConfig { dict }

    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        
        for (key, value) in &self.dict {
            result.extend_from_slice(format!("{} {}\n", key, value).as_bytes());
        }

        result
    }

    pub fn get_username(&self) -> String {
        let default = "Desconhecido".to_string();
        self.dict.get("username").unwrap_or(&default).clone()
    }

    pub fn get_email(&self) -> String {
        let default = "SEM EMAIL".to_string();
        self.dict.get("email").unwrap_or(&default).clone()
    }

    pub fn set(&mut self, key: String, value: String) {
        self.dict.insert(key, value);
    }
}