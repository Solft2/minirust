use crate::objects::RGitObject;

pub struct BlobObject {
    content: Vec<u8>
}

impl BlobObject {
    pub fn new(content: Vec<u8>) -> Self {
        BlobObject { content }
    }
}

impl RGitObject for BlobObject {
    fn serialize(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn deserialize(&mut self, object_bytes: Vec<u8>) {
        self.content = object_bytes;
    }

    fn object_type() -> &'static str {
        "blob"
    }
}