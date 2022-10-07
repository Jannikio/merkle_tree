use sha2::{Digest, Sha256};
use std::io;

pub trait Hasher {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut hashing = Sha256::new();
        hashing.update(&self.bytes())
    }
}
