use super::*;

pub struct Tree {
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Tree { nodes: vec![] }
    }
}
