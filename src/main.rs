use std::env::args;

use huffman_rs::tree::*;

fn main() {

    println!("\n\tHuffman Coding Demo:\n");

    let arg = args().collect::<Vec<String>>();
    println!("Args: {:?}", arg);

    let input = match arg.get(1) {
        Some(v) => v.clone(),
        None => "HELLO WORLD, THIS IS A TEST INPUT STRING. DO WITH THIS WHAT YOU WILL!".to_string(),
    };

    let second_arg = match arg.get(2) {
        Some(v) => v.clone(),
        None => input.clone(),
    };

    let tree = Tree::from_str(&input);

    // Displays the values
    for (k, v) in tree.make_table() {
        let mut out_str = String::new();
        for item in &v {
            match item {
                true => out_str += "1",
                false => out_str += "0",
            }
        }
        println!("{}: {:?}", k, out_str);
    }

    let (data, _table) = tree.encode(&second_arg.chars().collect::<Vec<char>>());

    let decoded = tree.decode(&data);

    for v in data {
        let mut out_str = String::new();
        match v {
            true => out_str += "1",
            false => out_str += "0",
        }
        print!("{}", out_str);
    }
    println!();
    let mut out_str = String::new();
    for v in decoded {
        out_str += &format!("{}", v);
    }
    println!("Decoded: `{}`", out_str);

    // let mut tree = Tree::new(Node::new(None, 2));
    // tree.left = Some(Arc::new(Tree::new(Node::new(None, 1))));
    // tree.right = Some(Arc::new(Tree::new(Node::new(None, 1))));

    // println!("{}", tree);

}
