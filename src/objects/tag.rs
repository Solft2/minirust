use crate::objects::RGitObject;

pub struct TagObject {
    content: Vec<u8>
}

impl TagObject {
    pub fn new(object_bytes: Vec<u8>) -> Self {
        panic!("Não implementado")
    }
}

impl RGitObject for TagObject {
    fn hash(&self) -> String {
        panic!("Não implementado")
    }

    fn serialize(&self) -> Vec<u8> {
        panic!("Não implementado");
    }

    #[allow(unused_variables)]
    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        panic!("Não implementado");
    }
}