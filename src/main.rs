use merkletreelib::*;

fn main() {

    // Generate a vector with 4104 values
    let mut values = Vec::new();
    let mut index = 0;
    while index < 4104 {
        let value = "V".to_string() + &index.to_string();
        values.push(value);
        index += 1;
    }
    
    // Create a Tree with a height of 10 and a arity of 8
    let tree = Tree::new(values, 8);

    // Get and Print the height and the root of the Tree
    let height = tree.get_height();
    let root = tree.get_root();
    println!("Root: {}, Height: {}", root, height);
        
}
