use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
};
#[derive(Debug)]
struct Node {
    hash: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(left: Node, right: Node, hash: u64) -> Node {
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            hash,
        }
    }
}

#[derive(Debug)]
pub struct MerkleTree {
    root: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(data: &[&[u8]]) -> Self {
        assert!(data.len().is_power_of_two());

        let mut hasher = DefaultHasher::new();
        let mut hash;
        let mut leafs = Vec::new();

        for block in data {
            hasher.write(block);
            hash = hasher.finish();
            leafs.push(Node {
                hash,
                right: None,
                left: None,
            })
        }

        let mut len = leafs.len();
        let mut nodes = VecDeque::from(leafs);
        let mut left: Node;
        let mut right: Node;

        while len > 1 {
            for _ in 0..len / 2 {
                left = nodes.pop_front().unwrap();
                right = nodes.pop_front().unwrap();
                hasher.write_u64(left.hash);
                hasher.write_u64(right.hash);
                hash = hasher.finish();
                nodes.push_back(Node::new(left, right, hash));
            }
            len = len / 2;
        }

        let root = nodes.pop_front().unwrap();
        MerkleTree { root: Some(Box::new(root)) }
    }
}
