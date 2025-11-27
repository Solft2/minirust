use crate::{objects::RGitObject, utils::{MultipleValuesMap, key_value_parse, key_value_serialize}};

pub struct CommitObject {
    content: MultipleValuesMap
}

impl CommitObject {
    pub fn new(content_bytes: Vec<u8>) -> Self{
        CommitObject {
            content: Self::content_from_bytes(content_bytes)
        }
    }

    fn content_from_bytes(bytes: Vec<u8>) -> MultipleValuesMap {
        let helper = String::from_utf8(bytes).expect("O objeto deve ser uma string UTF-8 válida");
        key_value_parse(&helper)
    }
}

impl RGitObject for CommitObject {
    fn serialize(&self) -> Vec<u8> {
        let result = key_value_serialize(&self.content);
        result.as_bytes().to_vec()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        self.content = Self::content_from_bytes(object_bytes);
    }
}