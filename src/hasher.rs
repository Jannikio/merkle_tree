use digest::DynDigest;

pub trait Hasher {
  
    fn use_hasher_leaf(hasher: &mut dyn DynDigest, data: &[u8]) -> Box<[u8]> {
        hasher.update(data);
        hasher.finalize_reset()
    }

    fn use_hasher_node(hasher: &mut dyn DynDigest, left: &[u8], right: &[u8]) -> Box<[u8]> {
        hasher.update(left);
        hasher.update(right);
        hasher.finalize_reset()
    }

    
    fn select_hasher(hash_function: &str) -> Box<dyn DynDigest> {
        match hash_function {
            "md5" => Box::new(md5::Md5::default()),
            "sha1" => Box::new(sha1::Sha1::default()),
            "sha224" => Box::new(sha2::Sha224::default()),
            "sha256" => Box::new(sha2::Sha256::default()),
            "sha384" => Box::new(sha2::Sha384::default()),
            "sha512" => Box::new(sha2::Sha512::default()),
            _ => unimplemented!("unsupported digest: {}", hash_function),
        }
    }

}
