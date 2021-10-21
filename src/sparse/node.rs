use fuel_storage::Storage;
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::Range;

use crate::common::{Bytes1, Bytes32, LEAF, NODE};
use crate::sparse::hash::sum;

// For a leaf:
// 00 - 01: Prefix (1 byte, 0x00)
// 01 - 33: hash(Key) (32 bytes)
// 33 - 65: hash(Data) (32 bytes)
//
// For a node:
// 00 - 01: Prefix (1 byte, 0x01)
// 01 - 32: Left child key (32 bytes)
// 33 - 65: Right child key (32 bytes)
//
const BUFFER_SZ: usize = size_of::<Bytes1>() + size_of::<Bytes32>() + size_of::<Bytes32>();
type Buffer = [u8; BUFFER_SZ];

#[derive(Clone, Debug)]
pub struct Node {
    buffer: Buffer,
}

impl Node {
    pub fn create_leaf(key: &[u8], data: &[u8]) -> Self {
        let buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        node.set_bytes_prefix(&[LEAF]);
        node.set_bytes_lo(&sum(key));
        node.set_bytes_hi(&sum(data));
        node
    }

    pub fn create_node(left_child_key: &Bytes32, right_child_key: &Bytes32) -> Self {
        let buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        node.set_bytes_prefix(&[NODE]);
        node.set_bytes_lo(left_child_key);
        node.set_bytes_hi(right_child_key);
        node
    }

    pub fn from_buffer(buffer: Buffer) -> Self {
        let node = Self { buffer };
        assert!(node.is_leaf() || node.is_node());
        node
    }

    pub fn prefix(&self) -> u8 {
        self.bytes_prefix()[0]
    }

    pub fn leaf_key(&self) -> &Bytes32 {
        assert!(self.is_leaf());
        self.bytes_lo().try_into().unwrap()
    }

    pub fn leaf_data(&self) -> &Bytes32 {
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

    pub fn is_leaf(&self) -> bool {
        self.prefix() == LEAF
    }

    pub fn is_node(&self) -> bool {
        self.prefix() == NODE
    }

    pub fn as_buffer(&self) -> &Buffer {
        self.buffer().try_into().unwrap()
    }

    pub fn value(&self) -> Bytes32 {
        sum(self.buffer())
    }

    // PRIVATE

    // PREFIX

    const fn prefix_offset() -> usize {
        0
    }

    const fn prefix_size() -> usize {
        size_of::<u8>()
    }

    const fn prefix_range() -> Range<usize> {
        Self::prefix_offset()..(Self::prefix_offset() + Self::prefix_size())
    }

    // BYTES LO

    const fn bytes_lo_offset() -> usize {
        Self::prefix_offset() + Self::prefix_size()
    }

    const fn bytes_lo_size() -> usize {
        size_of::<Bytes32>()
    }

    const fn bytes_lo_range() -> Range<usize> {
        Self::bytes_lo_offset()..(Self::bytes_lo_offset() + Self::bytes_lo_size())
    }

    // BYTES HI

    const fn bytes_hi_offset() -> usize {
        Self::bytes_lo_offset() + Self::bytes_lo_size()
    }

    const fn bytes_hi_size() -> usize {
        size_of::<Bytes32>()
    }

    const fn bytes_hi_range() -> Range<usize> {
        Self::bytes_hi_offset()..(Self::bytes_hi_offset() + Self::bytes_hi_size())
    }

    // BUFFER

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

    fn set_bytes_prefix(&mut self, bytes: &Bytes1) {
        self.bytes_prefix_mut().clone_from_slice(bytes);
    }

    fn bytes_lo_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_lo_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_lo(&self) -> &[u8] {
        let range = Self::bytes_lo_range();
        &self.buffer()[range]
    }

    fn set_bytes_lo(&mut self, bytes: &Bytes32) {
        self.bytes_lo_mut().clone_from_slice(bytes);
    }

    fn bytes_hi_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_hi_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_hi(&self) -> &[u8] {
        let range = Self::bytes_hi_range();
        &self.buffer()[range]
    }

    fn set_bytes_hi(&mut self, bytes: &Bytes32) {
        self.bytes_hi_mut().clone_from_slice(bytes);
    }
}

type NodeStorage<StorageError> = dyn Storage<Bytes32, Buffer, Error = StorageError>;

#[derive(Clone)]
struct StorageNode<'storage, StorageError> {
    storage: &'storage NodeStorage<StorageError>,
    node: Node,
}

impl<'a, 'storage, StorageError> StorageNode<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    pub fn new(storage: &'storage NodeStorage<StorageError>, node: Node) -> Self {
        Self { node, storage }
    }

    pub fn value(&self) -> Bytes32 {
        self.node.value()
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_leaf()
    }

    pub fn left_child(&'a self) -> Option<Self> {
        if self.node.is_node() {
            let key = self.node.left_child_key();
            let buffer = self.storage.get(key).unwrap().unwrap();
            let node = Node::from_buffer(*buffer);
            let storage_node = Self::new(self.storage, node);
            Some(storage_node)
        } else {
            None
        }
    }

    pub fn right_child(&'a self) -> Option<Self> {
        if self.node.is_node() {
            let key = self.node.right_child_key();
            let buffer = self.storage.get(key).unwrap().unwrap();
            let node = Node::from_buffer(*buffer);
            let storage_node = Self::new(self.storage, node);
            Some(storage_node)
        } else {
            None
        }
    }
}

impl<'storage, StorageError> crate::common::Node for StorageNode<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    type Key = Bytes32;

    fn key(&self) -> Self::Key {
        StorageNode::value(self)
    }

    fn is_leaf(&self) -> bool {
        StorageNode::is_leaf(self)
    }
}

impl<'storage, StorageError> crate::common::ParentNode for StorageNode<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    fn left_child(&self) -> Self {
        StorageNode::left_child(self).unwrap()
    }

    fn right_child(&self) -> Self {
        StorageNode::right_child(self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{IntoPathIterator, StorageError, StorageMap};

    #[test]
    fn test() {
        let n = Node::create_node(&[0u8; 32], &[1u8; 32]);
        let prefix = n.prefix();
        let left = n.left_child_key();
        let right = n.right_child_key();
    }

    #[test]
    fn test_storage() {
        let mut s = StorageMap::<Bytes32, Buffer>::new();

        let leaf_0 = Node::create_leaf("Hello World".as_bytes(), &[0u8; 32]);
        s.insert(&leaf_0.value(), leaf_0.as_buffer());

        let leaf_1 = Node::create_leaf("Something else".as_bytes(), &[1u8; 32]);
        s.insert(&leaf_1.value(), leaf_1.as_buffer());

        let node_0 = Node::create_node(&leaf_0.value(), &leaf_1.value());
        s.insert(&node_0.value(), node_0.as_buffer());

        let storage_node = StorageNode::<StorageError>::new(&mut s, node_0);
        let r = storage_node.right_child().unwrap();

        println!("{:?}", r.node);
    }
}
