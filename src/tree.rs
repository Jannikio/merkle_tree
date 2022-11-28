use std::collections::BTreeMap;

use super::*;

#[derive(Debug)]
pub struct Tree {
    pub leafs: Vec<Node>,
    pub data: Vec<String>,
    pub height: usize,
    pub arity: usize,
    pub nodes: BTreeMap<usize,Vec<Node>>,
    pub root: Vec<Node>
}

impl Tree {
    pub fn new(data: Vec<String>, arity: usize) -> Tree {
        let height = (data.len() / arity) / 2;
        let mut leafs = Vec::new();
        for input in data.clone() {
            let leaf = Node::create_leaf(input);
            leafs.push(leaf);
        }
        let mut nodes = BTreeMap::new();
        //let mut root = Node::create_leaf("input".to_string());
        
        for leaf in leafs.clone() {
            println!("leafs: {:?}", leaf);
        }
        
        let mut above_level = height.clone();
        nodes.insert(above_level, leafs.clone());
        let mut level = Vec::new();
        while above_level > 0 {
            let current_level = nodes.get(&above_level).unwrap();
            for single_node in (0..current_level.len()).step_by(2) {
                let left = current_level.get(single_node).unwrap();
                let right = current_level.get(single_node + 1).unwrap_or(left);
                let node = Node::create_none_leaf(left.clone(),  right.clone());
                level.push(node);
            };
            //println!("Level: {:?}", level);
            above_level -= 1;
            nodes.insert(above_level, level.clone());
            level.clear();
        }
        let binding = nodes.clone();
        let ref_root = &*binding.get(&0).unwrap();
        let root = ref_root.clone();
        

        for val in nodes.clone() {
            println!("Node: {:?} \n", val);
        }
        
        Tree { leafs, data, height, arity, nodes, root }
    }
}
