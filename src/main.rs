use merkletreelib::*;

fn main() {

    
    // Generate a vector with 4104 values
    let values = (0..4104).map(|v| "V".to_string() + &v.to_string()).collect();
    
    // Create a Tree with a height of 10 and a arity of 8
    let tree = Tree::new(values, 8);

    // Get and Print the height and the root of the Tree
    let height = tree.get_height();
    let root = tree.get_root();
    println!("Root: {}, Height: {}", root, height);
        
}
