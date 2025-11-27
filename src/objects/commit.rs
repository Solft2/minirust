use crate::{objects::RGitObject, utils::{MultipleValuesMap, key_value_parse, key_value_serialize}};

pub struct CommitObject {
    content: MultipleValuesMap
}

impl RGitObject for CommitObject {
    fn hash(&self) -> String {
        panic!("Não implementado")
    }

    fn serialize(&self) -> Vec<u8> {
        let result = key_value_serialize(&self.content);
        result.as_bytes().to_vec()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        let helper = String::from_utf8(object_bytes).expect("O objeto deve ser uma string UTF-8 válida");
        let map = key_value_parse(&helper);
        self.content = map;
    }
}