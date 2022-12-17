use super::*;
#[derive(Debug, Clone)]
pub struct  Node {
    hash: Box<[u8]>,
    pub string_hash: String
}

/// Creates a leaf or none leaf node
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
        let data1 = &left.hash;
        let data2 = &right.hash;
        let mut hasher1 = Self::select_hasher("sha256");
        let hash = Self::use_hasher_node(&mut *hasher1, data1, data2);
        let string_hash = hex::encode(&hash);
        Node{ hash, string_hash }
    }

    /// Get the string value of a leaf or none leaf node
    pub fn get_string_value(&self) -> String {
        self.string_hash.clone()
        
    }
}

/// Implements Hasher trait for Node struct
impl Hasher for Node {}