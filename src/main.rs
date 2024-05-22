mod merkle_tree;
use merkle_tree::MerkleTree;

fn main() {
    let data = ["hello".as_bytes(), "world".as_bytes()];
    print!("{:?}", MerkleTree::new(&data));

}