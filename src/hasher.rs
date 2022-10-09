use crypto_hash::{digest, Algorithm};
pub trait Hasher {
    fn input(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        digest(Algorithm::SHA256, &self.input())
    }
}
