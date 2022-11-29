use super::*;
#[derive(Debug, Clone)]
pub struct  Node {
    hash: Box<[u8]>,
    pub string_hash: String
}

impl Node {
    pub fn create_leaf(input: String) -> Node 
    {
        let data = input.as_bytes();
        let mut hasher1 = Self::select_hasher("sha256");
        let hash = Self::use_hasher(&mut *hasher1, data);
        let string_hash = hex::encode(hash.clone());
        Node{hash, string_hash}
        
    }
    
    pub fn create_none_leaf(left: Node, right: Node) -> Node
    {
        let data1 = &left.hash;
        let data2 = &right.hash;
        let mut hasher1 = Self::select_hasher("sha256");
        let hash = Self::use_hasher_node(&mut *hasher1, data1, data2);
        let string_hash = hex::encode(hash.clone());
        Node{ hash, string_hash }
    }

    pub fn get_string_value(&self) -> String {
        self.string_hash.clone()
        
    }
}

impl Hasher for Node {}