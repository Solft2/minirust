use crate::objects::RGitObject;

pub struct BlobObject<'a> {
    content: &'a [u8]
}

impl<'a> RGitObject<'a> for BlobObject::<'a> {
    fn hash(&self) -> &str {
        "Não implementado"
    }

    fn serialize(&self) -> &'a [u8] {
        self.content
    }

    fn deserialize(&mut self, object_bytes: &'a [u8]) {
        self.content = object_bytes;
    }
}