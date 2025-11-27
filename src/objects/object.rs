pub trait RGitObject {
    /// Hash do objeto
    fn hash(&self) -> String;

    /// Gere o conteúdo do arquivo do objeto
    fn serialize(&self) -> Vec<u8>;

    /// Crie o objeto a partir do conteúdo do arquivo que o representa
    fn deserialize(&mut self, object_bytes: Vec<u8>);

    fn get_object_bytes(&self) -> Vec<u8> {
        let mut content_bytes = self.serialize();
        let header = format!("{} {}", Self::object_type(), content_bytes.len());
        let mut object_bytes = header.as_bytes().to_vec();

        object_bytes.push(0x0);
        object_bytes.append(&mut content_bytes);

        object_bytes
    }

    fn object_type() -> &'static str {
        panic!("Não implementado")
    }
}