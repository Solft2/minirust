use crate::objects::RGitObject;

pub struct CommitObject<'a> {
    content: &'a [u8]
}

impl<'a> RGitObject<'a> for CommitObject::<'a> {
    fn hash(&self) -> &str {
        "Não implementado"
    }

    fn serialize(&self) -> &'a [u8] {
        panic!("Não implementado");
    }

    #[allow(unused_variables)]
    fn deserialize(&mut self, object_bytes: &'a [u8]) {
        panic!("Não implementado");
    }
}