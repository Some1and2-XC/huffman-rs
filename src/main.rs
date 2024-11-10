use std::sync::Arc;

use huffman_rs::tree::*;

fn main() {

    let tree = Tree::from_str("HELLO WORLD, HOW ARE WE ALL DOING TODAY? I LOVE YOU ALL AND HAVE A GOOD NIGHT!");

    // let mut tree = Tree::new(Node::new(None, 2));
    // tree.left = Some(Arc::new(Tree::new(Node::new(None, 1))));
    // tree.right = Some(Arc::new(Tree::new(Node::new(None, 1))));

    println!("{}", tree);

}
