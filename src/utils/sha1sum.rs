use sha1::{Digest, Sha1};

pub fn sha1sum(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}