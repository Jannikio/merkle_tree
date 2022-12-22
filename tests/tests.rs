#![cfg(test)]

use merkletreelib::*;

#[test]
fn test_create_new_tree() {
    let values = vec![
            "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
            ];
    let tree = Tree::new(values.clone(), 1);
    assert_eq!(values.len(), 4);
    assert_eq!(tree.get_height(), 2);
    assert_eq!(tree.get_data(), values);
    assert_eq!(tree.get_arity(), 1);
}

#[test]
fn test_create_new_tree_with_arity_2() {
    let values = vec![
            "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
            ];
    let tree = Tree::new(values.clone(), 2);
    let arity_values = vec!["V0V1".to_string(), "V2V3".to_string()];
    assert_eq!(tree.get_height(), 1);
    assert_eq!(tree.get_data(), arity_values);
    assert_eq!(tree.get_arity(), 2);
}

#[test]
fn test_create_new_tree_of_int() {
    let values = vec![0,1,2,3];
    let tree = Tree::new(values.clone(), 1);
    assert_eq!(values.len(), 4);
    assert_eq!(tree.get_height(), 2);
    assert_eq!(tree.get_arity(), 1);
}
 
#[test]
fn test_get_leafs() {
    let v0 = "bc946d74e8aee763e228fa42d6f4bcf3f164e54c769f806c3d22e544bbd8e424".to_string();
    let v1 = "dae96fc0046a5ae1864d9a66e6715da8da08240e7119816ab722261c0744d8e8".to_string();
    let v2 = "47cfe5eb8ada0e7492d86a6b47dc93e354c32af4a3cf489e0c65067cb48277d9".to_string();
    let v3 = "8b336cdf52ac9b7597ea56288537edde99faf2911423e321d811751b9fcfbd5f".to_string();
    let test_leafs = vec![v0, v1, v2, v3];
    let values = vec![
            "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
            ];
    let tree = Tree::new(values, 1);
    let leafs = tree.get_leafs();
    assert_eq!(leafs, test_leafs);
}

 
#[test]
fn test_get_root() {
    let values = vec![
        "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
        ];
    let tree = Tree::new(values, 1);
    let root = tree.get_root();
    let test_root = "cbb27bd05042177bf759e4530b10438b1748d71014cf3fc68bca522d20d422b4".to_string();
    assert_eq!(root, test_root);
}

 
#[test]
fn test_get_nodes() {
    let values = vec![
        "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
        ];
    let tree = Tree::new(values, 1);
    let nodes = tree.get_nodes();
    let node0 = "732d279c9ffd8302dded1c291c7b623dd385b2e7241dd0c891fbaddb8033efff".to_string();
    let node1 = "62f142501512cf5a011dc3297d47315a5fb0bba09dc0044c418c2df37d8a51b7".to_string();
    let test_nodes = vec![node0, node1];
    assert_eq!(nodes, test_nodes);
}

 
#[test]
fn test_get_opening() {
    let values = vec![
        "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
        ];
    let tree = Tree::new(values, 1);
    let opening = tree.get_opening(1);
    let string_opening:Vec<String> = opening.iter().map(|opening| opening.clone().get_string_value()).collect();
    let test_value_1 = "bc946d74e8aee763e228fa42d6f4bcf3f164e54c769f806c3d22e544bbd8e424".to_string();
    let test_value_2 = "62f142501512cf5a011dc3297d47315a5fb0bba09dc0044c418c2df37d8a51b7".to_string();
    let test_opening = vec![test_value_1, test_value_2];
    assert_eq!(string_opening, test_opening);
}


#[test]
fn test_get_leaf_index_by_values() {
    let values = vec![
        "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
        ];
    let tree = Tree::new(values, 1);
    let index = tree.get_leaf_index_by_values("V2".to_string());
    assert_eq!(index, 2);
}


#[test]
fn test_get_values_from_leaf() {
    let values = vec![
        "V0".to_string(), "V1".to_string(), "V2".to_string(), "V3".to_string()
        ];
    let tree = Tree::new(values, 1);
    let leaf_values = tree.get_values_from_leaf(1);
    assert_eq!(leaf_values, "V1".to_string());
}

