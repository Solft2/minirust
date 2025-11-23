use crate::objects::RGitObject;

pub struct TreeObject {
    content: Vec<u8>
}

impl RGitObject for TreeObject {
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