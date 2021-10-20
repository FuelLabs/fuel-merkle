use digest::Digest;
use fuel_storage::Storage;
use std::cell::Cell;
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::Range;

use crate::common::{ParentNode, Position, StorageError};

use sha2::Sha256 as HasBh;

const NODE: u8 = 0x01;
const LEAF: u8 = 0x00;

// For a node:
// 00 - 01: Prefix (0x01)
// 01 - 33: Key
// 33 - 65: hash(Data)
//
// For a leaf:
// 00 - 01: Prefix (0x00)
// 01 - 32: Left child key
// 33 - 65: Right child key
//
type Bytes32 = [u8; 32];
type Buffer = [u8; 1 + size_of::<Bytes32>() + size_of::<Bytes32>()];

#[derive(Clone)]
pub struct Node {
    buffer: [u8; 65]
}

impl Node {
    pub fn create_leaf(key: &Bytes32, data: &Bytes32) -> Self {
        let mut buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        node.bytes_prefix_mut().clone_from_slice(&[LEAF]);
        node.bytes_lo_mut().clone_from_slice(key);
        node.bytes_hi_mut().clone_from_slice(data);
        node
    }

    pub fn create_node(left_child_key: &Bytes32, right_child_key: &Bytes32) -> Self {
        let mut buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        node.bytes_prefix_mut().clone_from_slice(&[NODE]);
        node.bytes_lo_mut().clone_from_slice(left_child_key);
        node.bytes_hi_mut().clone_from_slice(right_child_key);
        node
    }

    pub fn from_buffer(buffer: &Buffer) -> Self {
        let node = Self { buffer: buffer.clone() };
        assert!(node.is_leaf() || node.is_node());
        node
    }

    pub fn prefix(&self) -> u8 {
        self.bytes_prefix()[0]
    }

    pub fn key(&self) -> &Bytes32 {
        assert!(self.is_leaf());
        self.bytes_lo().try_into().unwrap()
    }

    pub fn data(&self) -> &Bytes32 {
        assert!(self.is_leaf());
        self.bytes_hi().try_into().unwrap()
    }

    pub fn left_child_key(&self) -> &Bytes32 {
        assert!(self.is_node());
        self.bytes_lo().try_into().unwrap()
    }

    pub fn right_child_key(&self) -> &Bytes32 {
        assert!(self.is_node());
        self.bytes_hi().try_into().unwrap()
    }

    pub fn value(&self) -> &Buffer {
        self.buffer().try_into().unwrap()
    }

    pub fn is_leaf(&self) -> bool {
        self.prefix() == LEAF
    }

    pub fn is_node(&self) -> bool {
        self.prefix() == NODE
    }

    // PRIVATE

    const fn prefix_offset() -> usize {
        0
    }

    const fn prefix_size() -> usize {
        size_of::<u8>()
    }

    const fn prefix_range() -> Range<usize> {
        Self::prefix_offset()..(Self::prefix_offset() + Self::prefix_size())
    }

    const fn bytes_lo_offset() -> usize {
        Self::prefix_offset() + Self::prefix_size()
    }

    const fn bytes_lo_size() -> usize {
        size_of::<Bytes32>()
    }

    const fn bytes_lo_range() -> Range<usize> {
        Self::bytes_lo_offset()..(Self::bytes_lo_offset() + Self::bytes_lo_size())
    }

    const fn bytes_hi_offset() -> usize {
        Self::bytes_lo_offset() + Self::bytes_lo_size()
    }

    const fn bytes_hi_size() -> usize {
        size_of::<Bytes32>()
    }

    const fn bytes_hi_range() -> Range<usize> {
        Self::bytes_hi_offset()..(Self::bytes_hi_offset() + Self::bytes_hi_size())
    }

    const fn buffer_size() -> usize {
        Self::prefix_size() + Self::bytes_lo_size() + Self::bytes_hi_size()
    }

    // PRIVATE

    fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn bytes_prefix_mut(&mut self) -> &mut [u8] {
        let range = Self::prefix_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_prefix(&self) -> &[u8] {
        let range = Self::prefix_range();
        &self.buffer()[range]
    }

    fn bytes_lo_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_lo_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_lo(&self) -> &[u8] {
        let range = Self::bytes_lo_range();
        &self.buffer()[range]
    }

    fn bytes_hi_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_hi_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_hi(&self) -> &[u8] {
        let range = Self::bytes_hi_range();
        &self.buffer()[range]
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;
    use super::*;

    #[test]
    fn test() {
        let n = Node::create_node(&[0u8; 32], &[1u8; 32]);
        let prefix = n.prefix();
        println!("{:?}", prefix);
        let left = n.left_child_key();
        println!("{:?}", left);
        let right = n.right_child_key();
        println!("{:?}", right);

    }

}
