use crate::binary::{leaf_sum, node_sum, Buffer};
use crate::common::{Bytes32, Position};

use core::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
    buffer: Buffer,
}

impl Node {
    pub fn create_leaf(index: u64, data: &[u8]) -> Self {
        let mut buffer = Buffer::default();
        *buffer.position_mut() = Position::from_leaf_index(index);
        *buffer.hash_mut() = leaf_sum(data);
        Self { buffer }
    }

    pub fn create_node(left_child: &Self, right_child: &Self) -> Self {
        let mut buffer = Buffer::default();
        *buffer.position_mut() = left_child.position().parent();
        *buffer.hash_mut() = node_sum(left_child.hash(), right_child.hash());
        Self { buffer }
    }

    pub fn position(&self) -> Position {
        self.buffer.position()
    }

    pub fn key(&self) -> u64 {
        self.position().in_order_index()
    }

    pub fn hash(&self) -> &Bytes32 {
        self.buffer.hash()
    }
}

impl From<Buffer> for Node {
    fn from(buffer: Buffer) -> Self {
        Self { buffer }
    }
}
