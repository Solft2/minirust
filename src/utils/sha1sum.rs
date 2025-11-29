use sha1::{Digest, Sha1};

/// Hash em uma sequência de bytes
pub fn sha1sum(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn is_valid_sha1(hash: &str) -> bool {
    if hash.len() != 40 {
        return false;
    }

    hash.chars().all(|c| c.is_digit(16))
}