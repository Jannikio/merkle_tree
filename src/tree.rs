use std::collections::BTreeMap;

use super::*;

#[derive(Debug)]
pub struct Tree {
    pub leafs: Vec<Node>,
    pub data: Vec<String>,
    pub height: usize,
    pub arity: usize,
    pub nodes: BTreeMap<usize,Vec<Node>>,
    pub root: Node
}

impl Tree {
    pub fn new(data: Vec<String>, arity: usize) -> Tree {
        let height_f = (data.len() as f64).log2();
        let height = if height_f - height_f.floor() > 0.0 
                            {
                                (height_f + 1.0) as usize
                            } else {
                                height_f as usize
                            };
        let mut leafs = Vec::new();
        for input in data.clone() {
            let leaf = Node::create_leaf(input);
            leafs.push(leaf);
        }
        let mut nodes = BTreeMap::new();
        
        let mut tree_level = height.clone();
        nodes.insert(tree_level, leafs.clone());
        let mut level = Vec::new();
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
        let binding = nodes.clone();
        let ref_root = &*binding.get(&0).unwrap();
        let root = ref_root.first().unwrap().clone();
        
        Tree { leafs, data, height, arity, nodes, root }
    }

    pub fn get_height(&self) -> usize {
        self.height.clone()
    }
    pub fn get_data(&self) -> Vec<String> {
        self.data.clone()
    }
    pub fn get_leafs(&self) -> Vec<Node> {
        self.leafs.clone()
    }
    pub fn get_root(&self) -> String {
        self.root.clone().get_string_value()
    }
    pub fn get_nodes(&self) -> BTreeMap<usize,Vec<Node>> {
        self.nodes.clone()
    }
}
