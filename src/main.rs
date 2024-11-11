use huffman_rs::tree::*;

fn main() {

    let tree = Tree::from_str("HELLO WORLD, HOW ARE WE ALL DOING TODAY? I LOVE YOU ALL AND HAVE A GOOD NIGHT!");

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

    let (data, table) = tree.encode(&"HELLO WORLD".chars().collect::<Vec<char>>());
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
    println!("{}", out_str);


    // let mut tree = Tree::new(Node::new(None, 2));
    // tree.left = Some(Arc::new(Tree::new(Node::new(None, 1))));
    // tree.right = Some(Arc::new(Tree::new(Node::new(None, 1))));

    // println!("{}", tree);

}
