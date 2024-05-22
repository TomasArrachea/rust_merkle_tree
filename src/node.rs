#[derive(Debug, PartialEq)]
pub struct Node {
    pub hash: u64,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(left: Node, right: Node, hash: u64) -> Node {
        Node {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            hash,
        }
    }
}