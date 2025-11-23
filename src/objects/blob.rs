use crate::objects::RGitObject;
use crate::utils::sha1sum;

pub struct BlobObject {
    content: Vec<u8>
}

impl BlobObject {
    pub fn new(content: Vec<u8>) -> Self {
        BlobObject { content }
    }
}

impl RGitObject for BlobObject {
    fn hash(&self) -> String {
        sha1sum(&self.content)
    }

    fn serialize(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        self.content = object_bytes;
    }
}