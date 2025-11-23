pub trait RGitObject<'a> {
    fn hash(&self) -> &str;
    fn serialize(&self) -> &'a [u8];
    fn deserialize(&mut self, object_bytes: &'a [u8]);
}