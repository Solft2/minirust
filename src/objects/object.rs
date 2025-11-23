pub trait RGitObject {
    fn hash(&self) -> String;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, object_bytes: Vec<u8>);
}