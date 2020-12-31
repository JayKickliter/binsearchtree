use binsearchtree::LTree;

fn main() {
    let mut tree = LTree::new();
    tree.insert(5, ());
    tree.insert(3, ());
    tree.insert(1, ());
    tree.insert(2, ());
    tree.insert(7, ());
    tree.insert(6, ());
    tree.insert(8, ());
    println!("{:#?}", tree);

    let mut tree = LTree::new();
    for (v, k) in (0..100).into_iter().rev().enumerate() {
        tree.insert(k, v);
    }
    let iter = tree.iter();
    for kv in iter {
        print!("{:?} ", kv);
    }
}
