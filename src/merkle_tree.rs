use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hasher},
};
use crate::node::Node;

#[derive(Debug, PartialEq)]
pub struct MerkleTree {
    root: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(data: &[&[u8]]) -> Self {
        assert!(data.len().is_power_of_two());

        let mut hasher = DefaultHasher::new();
        let hashed_data = data.iter().map(|block| {
            hasher.write(block);
            hasher.finish()
        });
        let leafs: Vec<Node> = hashed_data
            .map(|hash| Node {
                hash,
                right: None,
                left: None,
            })
            .collect();
        let mut nodes = VecDeque::from(leafs);
        let mut level_size = nodes.len();
        let mut left: Node;
        let mut right: Node;
        let mut hash: u64;

        while level_size > 1 {
            for _ in 0..level_size / 2 {
                left = nodes.pop_front().unwrap();
                right = nodes.pop_front().unwrap();
                hasher.write_u64(left.hash);
                hasher.write_u64(right.hash);
                hash = hasher.finish();
                nodes.push_back(Node::new(left, right, hash));
            }
            level_size = level_size / 2;
        }

        let root = nodes.pop_front().unwrap();
        MerkleTree {
            root: Some(Box::new(root)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_bytes(hasher: &mut dyn Hasher, data: &[u8]) -> u64{
        hasher.write(data);
        hasher.finish()
    }

    #[test]
    #[should_panic]
    fn test_create_fails_if_data_empty() {
        MerkleTree::new(&[]);
    }

    #[test]
    #[should_panic]
    fn test_create_fails_if_data_is_not_power_of_two() {
        let data = ["1".as_bytes(), "2".as_bytes(), "3".as_bytes()];
        MerkleTree::new(&data);
    }

    #[test]
    fn test_create_succesfull() {
        let hasher = DefaultHasher::new();
        let data = ["block_1".as_bytes(), "block_2".as_bytes(), "block_3".as_bytes(), "block_4".as_bytes()];

        hasher.write(data[0]);
        let hash_1 = hasher.finish();
        hasher.write(data[1]);
        let hash_2 = hasher.finish();
        hasher.write(data[2]);
        let hash_3 = hasher.finish();
        hasher.write(data[3]);
        let hash_4 = hasher.finish();

        let node_1 = Node { hash: hash_1, left: None, right: None };
        let node_2 = Node { hash: hash_2, left: None, right: None };
        let node_3 = Node { hash: hash_3, left: None, right: None };
        let node_4 = Node { hash: hash_4, left: None, right: None };

        hasher.write_u64(hash_1);
        hasher.write_u64(hash_2);
        let hash_1_2 = hasher.finish();
        
        let node_1_2 = Node { hash: hash_1_2, left: node_1, right: node_2 };
        let expected_tree = MerkleTree {
            root: Node {
                hash:,

            }
        }
        assert_eq!(List::Nil, create_empty_list())
    }
}
