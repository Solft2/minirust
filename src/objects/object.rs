pub trait RGitObject {
    /// Hash do objeto
    fn hash(&self) -> String;

    /// Gere o conteúdo do arquivo do objeto
    fn serialize(&self) -> Vec<u8>;

    /// Crie o objeto a partir do conteúdo do arquivo que o representa
    fn deserialize(&mut self, object_bytes: Vec<u8>);
}