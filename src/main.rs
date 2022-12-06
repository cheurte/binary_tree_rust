use binary_tree::Node;
use std::ops;
use rand::Rng;

fn main() {
    let mut tree = Node::new(500);

    for _ in (ops::Range{start:1, end:50}){
        let val:i32 = rand::thread_rng().gen_range(1..=1000);
        tree.insert(val);
        assert_eq!(true, tree.search(val));
    } 
    
    print!("{}",tree);

}

