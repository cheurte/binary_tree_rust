use binary_tree::Node;

#[test]
fn insert_test(){
    let mut tree = Node::new(5);
    tree.insert(7);
    assert_eq!(7, tree.right_branch.unwrap().value);
}
#[test]
fn search_test(){
    let mut tree = Node::new(5);
    tree.insert(7);
    assert_eq!(true, tree.search(7));
}