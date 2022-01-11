use crate::binary::{leaf_sum, node_sum};
use std::fmt::Debug;

use crate::common::{Bytes32, Position};

#[derive(Clone, PartialEq, Debug)]
pub struct Node {
    position: Position,
    hash: Bytes32,
}

impl Node {
    pub fn create_leaf(index: u64, data: &[u8]) -> Self {
        let position = Position::from_leaf_index(index);
        let hash = leaf_sum(data);
        Self {
            position,
            hash
        }
    }

    pub fn create_node(left_child: &mut Self, right_child: &mut Self) -> Self {
        let position = left_child.position().parent();
        let hash = node_sum(left_child.hash(), right_child.hash());
        Self {
            position,
            hash
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn key(&self) -> u64 {
        self.position().in_order_index()
    }

    pub fn left_key(&self) -> u64 {
        self.position().left_child().in_order_index()
    }

    pub fn right_key(&self) -> u64 {
        self.position().right_child().in_order_index()
    }

    pub fn hash(&self) -> &Bytes32 {
        &self.hash
    }
}
