use merkletreelib::*;

fn main() {
    println!("Hello, world!");
    let tree = Tree::new(vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(),
                                          "7".to_string(), "8".to_string(), "9".to_string(), "10".to_string(), "11".to_string(), "12".to_string(),
                                          "13".to_string(), "14".to_string(), "15".to_string(), "16".to_string(), "17".to_string(), "18".to_string(), 
                                          "19".to_string(), "20".to_string(), "21".to_string(), "22".to_string(), "23".to_string(), "24".to_string()], 9);

    let height = tree.get_height();
    let data = tree.get_data();
    let leafs = tree.get_leafs();
    let root = tree.get_root();
    let nodes = tree.get_nodes();

    println!("Root: {}, Height: {}", root, height);

    for val in nodes {
        println!("Node: {:?} \n", val);
    }
    
}
