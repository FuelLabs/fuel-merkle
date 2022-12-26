use crate::{
    binary::{
        buffer::{Buffer, ReadView, WriteView, DEFAULT_BUFFER},
        leaf_sum, node_sum,
    },
    common::{Bytes32, Position},
};

use core::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Node {
    buffer: Buffer,
}

impl Node {
    pub fn create_leaf(index: u64, data: &[u8]) -> Self {
        let mut buffer = *DEFAULT_BUFFER;
        let mut view = WriteView::new(&mut buffer);
        *view.position_mut() = Position::from_leaf_index(index);
        *view.hash_mut() = leaf_sum(data);
        Self { buffer }
    }

    pub fn create_node(left_child: &Self, right_child: &Self) -> Self {
        let mut buffer = *DEFAULT_BUFFER;
        let mut view = WriteView::new(&mut buffer);
        *view.position_mut() = left_child.position().parent();
        *view.hash_mut() = node_sum(left_child.hash(), right_child.hash());
        Self { buffer }
    }

    pub fn position(&self) -> Position {
        let view = ReadView::new(&self.buffer);
        view.position()
    }

    pub fn key(&self) -> u64 {
        self.position().in_order_index()
    }

    pub fn hash(&self) -> &Bytes32 {
        let view = ReadView::new(&self.buffer);
        let ptr = view.hash() as *const Bytes32;
        // SAFETY: ptr is guaranteed to point to a valid range of 32 bytes owned
        //         by self.buffer
        unsafe { &*ptr }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

impl From<Buffer> for Node {
    fn from(buffer: Buffer) -> Self {
        Self { buffer }
    }
}

impl From<Node> for Buffer {
    fn from(node: Node) -> Self {
        node.buffer
    }
}
