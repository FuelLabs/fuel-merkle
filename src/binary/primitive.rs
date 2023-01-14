use crate::{binary::Node, common::Position};

pub type Primitive = [u8; 40];

impl From<&Node> for Primitive {
    fn from(node: &Node) -> Self {
        let mut primitive = [0u8; 40];
        primitive[0..8].copy_from_slice(&node.position().in_order_index().to_be_bytes());
        primitive[8..40].copy_from_slice(node.hash());
        primitive
    }
}

impl From<&Primitive> for Node {
    fn from(primitive: &Primitive) -> Self {
        let mut position_array = [0u8; 8];
        position_array.copy_from_slice(&primitive[0..8]);
        let position = Position::from_in_order_index(u64::from_be_bytes(position_array));
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&primitive[8..40]);
        Node::new(position, hash)
    }
}
