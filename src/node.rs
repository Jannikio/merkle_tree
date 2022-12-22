use digest::DynDigest;

#[derive(Debug, Clone)]
pub struct  Node {
    hash: Box<[u8]>,
    string_hash: String
}

// Creates a leaf or none leaf node
impl Node {

    // Create leaf from string input
    pub fn create_leaf(input: &String) -> Node 
    {
        let data = input.as_bytes();
        let mut hasher1 = Self::select_hasher("sha256");
        let hash = Self::use_hasher_leaf(&mut *hasher1, data);
        let string_hash = hex::encode(&hash);
        Node{hash, string_hash}
        
    }
    
    // Create none leaf from input of two nodes
    pub fn create_none_leaf(left: &Node, right: &Node) -> Node
    {
        let data_left = &left.hash;
        let data_right = &right.hash;
        let mut hasher1 = Self::select_hasher("sha256");
        let hash = Self::use_hasher_node(&mut *hasher1, data_left, data_right);
        let string_hash = hex::encode(&hash);
        Node{ hash, string_hash }
    }

    // Create hash for leaf
    fn use_hasher_leaf(hasher: &mut dyn DynDigest, data: &[u8]) -> Box<[u8]> {
        hasher.update(data);
        hasher.finalize_reset()
    }
    
    // Create hash for none leaf
    fn use_hasher_node(hasher: &mut dyn DynDigest, left: &[u8], right: &[u8]) -> Box<[u8]> {
        hasher.update(left);
        hasher.update(right);
        hasher.finalize_reset()
    }
    
    // Select a hasher
    fn select_hasher(hash_function: &str) -> Box<dyn DynDigest> {
        match hash_function {
            "sha256" => Box::new(sha2::Sha256::default()),
            "sha512" => Box::new(sha2::Sha512::default()),
            _ => unimplemented!("unsupported digest: {}", hash_function),
        }
    }

    // Get the string value of a leaf or none leaf node
    pub fn get_string_value(&self) -> String {
        self.string_hash.clone()
        
    }
}