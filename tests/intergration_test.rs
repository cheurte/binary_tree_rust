use binary_tree::Node;

#[test]
fn search_test(){
    let mut tree = Node::new(5);
    tree.insert(7);
    assert_eq!(true, tree.search(7));
}