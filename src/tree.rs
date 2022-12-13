use std::collections::BTreeMap;

use super::*;

#[derive(Debug)]
pub struct Tree {
    pub inputs: Vec<String>,
    pub data: Vec<String>,
    pub height: usize,
    pub arity: usize,
    pub nodes: BTreeMap<usize,Vec<Node>>
}

impl Tree {

    /// Creates a new Tree with given values
    /// # Example
    /// use merkletreelib::*;
    /// let values = vec!["V0".to_string(), 
    ///                   "V1".to_string(), 
    ///                   "V2".to_string(), 
    ///                   "V3".to_string()];
    /// let tree = Tree::new(values, 1);
    /// let test_root = "cbb27bd05042177bf759e4530b10438b1748d71014cf3fc68bca522d20d422b4"
    /// assert_eq!(tree.get_root, test_root);
    pub fn new(inputs: Vec<String>, arity: usize) -> Tree { 
        // Calculates the height of the Tree
        let length = inputs.len() / arity;
        let height_f = (length as f64).log2();
        let height = if height_f - height_f.floor() > 0.0 
                            {
                                (height_f + 1.0) as usize
                            } else {
                                height_f as usize
                            };

        // Applys arity to the input values
        if (inputs.len() % arity) > 0 {
            panic!("not valide arity");
        }  
        let mut values = Vec::new();
        let mut data = Vec::new();
        let mut current_arity = 0;
        for input in (0..inputs.len()).step_by(arity) {
            while current_arity != arity {
                let value = inputs.get(input + current_arity).unwrap();
                let deref_value = value.clone();
                values.push(deref_value);
                current_arity += 1;
            }
            let concat_values = values.concat();
            values.clear();
            current_arity = 0;
            data.push(concat_values);
        }

        // Generates leafs out of input data
        let mut leafs = Vec::new();
        for input in data.clone() {
            let leaf = Node::create_leaf(input);
            leafs.push(leaf);
        }
        let mut nodes = BTreeMap::new();
        let mut tree_level = height.clone();
        nodes.insert(tree_level, leafs);
        let mut level = Vec::new();

        // Creates the Tree
        while tree_level > 0 {
            let current_level = nodes.get(&tree_level).unwrap();
            for single_node in (0..current_level.len()).step_by(2) {
                let left = current_level.get(single_node).unwrap();
                let right = current_level.get(single_node + 1).unwrap_or(left);
                let node = Node::create_none_leaf(left.clone(),  right.clone());
                level.push(node);
            };
            tree_level -= 1;
            nodes.insert(tree_level, level.clone());
            level.clear();
        }
        
        Tree { inputs, data , height, arity, nodes }
    }

    pub fn get_height(&self) -> usize {
        self.height.clone()
    }
    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }

    /// Get the string values of the leaf nodes
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let leafs = tree.get_leafs();
    /// for leaf in leafs {
    ///     println!("Leaf: {} \n", leaf);
    /// }
    pub fn get_leafs(&self) -> Vec<String> {
        let binding = self.nodes.clone();
        let raw_leafs = binding.iter().next_back();
        let leafs = raw_leafs.unwrap().1.clone();
        let mut leaf_string = Vec::new();
        for leaf in leafs {
            let string = leaf.string_hash;
            leaf_string.push(string);
        }
        leaf_string
        
    }

    /// Get the root value
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let root = tree.get_root();
    pub fn get_root(&self) -> String {
        let binding = self.nodes.clone();
        let ref_root = &*binding.get(&0).unwrap();
        let root = ref_root.first().unwrap().clone();
        root.get_string_value()
    }

    /// Get the string value of none leaf nodes
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let nodes = tree.get_nodes();
    /// for node in nodes {
    ///     println!("Node: {} \n", node);
    /// }
    pub fn get_nodes(&self) -> Vec<String> {
        let get_nodes = self.nodes.clone();
        let mut nodes_string = Vec::new();
        let mut level = self.height - 1;
        while level > 0 {
            let node = get_nodes.get(&level).unwrap();
            for val in node {
                let string = val.clone().string_hash;
                nodes_string.push(string);
            }
            level -= 1;
        }
        nodes_string

    }


    /// Get opening of a leaf at a provided index
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let opening = tree.get_opening(0);
    pub fn get_opening(&self, index_e: usize) -> Vec<Node> {
        let mut index = index_e;
        let mut level = self.height;
        let mut opening = Vec::new();
        let nodes_len = self.nodes.len();
        if index_e > nodes_len  {
            panic!("Not valid entry");
        }
        while level > 0 {
            let nodes_opening = self.nodes.get(&level);
            for nodes_level in nodes_opening {
                if index % 2 == 0 {
                    let value = nodes_level.get(index + 1).unwrap();
                    index /= 2;
                    opening.push(value.clone());
                } else {
                    let value = nodes_level.get(index - 1).unwrap();
                    index /= 2;
                    opening.push(value.clone());
                } 
            }
            level -= 1;
        }
        opening
    }

    /// Get the index of a leaf by providing a input value
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let leaf_index = tree.get_leaf_index_by_values("V2".to_string());
    /// assert_eq!(leaf_index, 2);
    pub fn get_leaf_index_by_values(&self, input: String) -> usize {
        let leaf_data = self.data.clone();
        let index = leaf_data.iter().position(|r| *r == input).unwrap();
        index
        
    }

    /// Get input value by providing a index value
    /// # Example
    /// use merkletreelib::*;
    /// let tree = Tree::new(vec![
    ///                             "V0".to_string(), 
    ///                             "V1".to_string(), 
    ///                             "V2".to_string(), 
    ///                             "V3".to_string()
    ///                            ], 1);
    /// let leaf_values = tree.get_values_from_leaf(1);
    /// assert_eq!(leaf_values, "V1".to_string());
    pub fn get_values_from_leaf(&self, index: usize) -> String {
        let values = self.inputs.clone();
        values.get(index).unwrap().clone()
    }
}