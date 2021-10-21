use digest::Digest;
use fuel_storage::Storage;
use std::convert::TryInto;
use std::mem::size_of;
use std::ops::Range;

use crate::common::{ParentNode, Position, StorageError};

use sha2::Sha256 as Hash;

const NODE: u8 = 0x01;
const LEAF: u8 = 0x00;

// For a node:
// 00 - 01: Prefix (1 byte) (0x01)
// 01 - 33: Key (32 bytes)
// 33 - 65: hash(Data) (32 bytes)
// 65 - 73: Index (8 bytes)
// 73 - 77: Height (4 bytes)
//
// For a leaf:
// 00 - 01: Prefix (0x00)
// 01 - 32: Left child key
// 33 - 65: Right child key
// 65 - 73: Index (8 bytes)
// 73 - 77: Height (4 bytes)
//
type Bytes32 = [u8; 32];

const BUFFER_SZ: usize = size_of::<u8>()
    + size_of::<Bytes32>()
    + size_of::<Bytes32>()
    + size_of::<u64>()
    + size_of::<u32>();
type Buffer = [u8; BUFFER_SZ];

#[derive(Clone, Debug)]
pub struct Node {
    buffer: Buffer,
}

impl Node {
    pub fn create_leaf(key: &Bytes32, data: &Bytes32, index: u64) -> Self {
        let buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        let position = Position::from_in_order_index(index);
        node.bytes_prefix_mut().clone_from_slice(&[LEAF]);
        node.bytes_lo_mut().clone_from_slice(key);
        node.bytes_hi_mut().clone_from_slice(data);
        node.bytes_index_mut().clone_from_slice(&index.to_be_bytes());
        node.bytes_height_mut().clone_from_slice(&position.height().to_be_bytes());
        node
    }

    pub fn create_node(left_child_key: &Bytes32, right_child_key: &Bytes32, index: u64) -> Self {
        let buffer = [0u8; Self::buffer_size()];
        let mut node = Self { buffer };
        let position = Position::from_in_order_index(index);
        node.bytes_prefix_mut().clone_from_slice(&[NODE]);
        node.bytes_lo_mut().clone_from_slice(left_child_key);
        node.bytes_hi_mut().clone_from_slice(right_child_key);
        node.bytes_index_mut().clone_from_slice(&index.to_be_bytes());
        node.bytes_height_mut().clone_from_slice(&position.height().to_be_bytes());
        node
    }

    pub fn from_buffer(buffer: &Buffer) -> Self {
        let node = Self {
            buffer: buffer.clone(),
        };
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

    pub fn index(&self) -> u64 {
        let buffer: [u8; 8] = self.bytes_index().try_into().unwrap();
        u64::from_be_bytes(buffer)
    }

    pub fn height(&self) -> u32 {
        let buffer: [u8; 4] = self.bytes_height().try_into().unwrap();
        u32::from_be_bytes(buffer)
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
        let mut hash = Hash::new();
        hash.update(self.buffer());
        hash.finalize().try_into().unwrap()
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

    // BYTES INDEX

    const fn bytes_index_offset() -> usize {
        Self::bytes_hi_offset() + Self::bytes_hi_size()
    }

    const fn bytes_index_size() -> usize {
        size_of::<u64>()
    }

    const fn bytes_index_range() -> Range<usize> {
        Self::bytes_index_offset()..(Self::bytes_index_offset() + Self::bytes_index_size())
    }

    // BYTES HEIGHT

    const fn bytes_height_offset() -> usize {
        Self::bytes_index_offset() + Self::bytes_index_size()
    }

    const fn bytes_height_size() -> usize {
        size_of::<u32>()
    }

    const fn bytes_height_range() -> Range<usize> {
        Self::bytes_height_offset()..(Self::bytes_height_offset() + Self::bytes_height_size())
    }

    // BUFFER

    const fn buffer_size() -> usize {
        Self::prefix_size()
            + Self::bytes_lo_size()
            + Self::bytes_hi_size()
            + Self::bytes_index_size()
            + Self::bytes_height_size()
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

    fn bytes_index_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_index_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_index(&self) -> &[u8] {
        let range = Self::bytes_index_range();
        &self.buffer()[range]
    }

    fn bytes_height_mut(&mut self) -> &mut [u8] {
        let range = Self::bytes_height_range();
        &mut self.buffer_mut()[range]
    }

    fn bytes_height(&self) -> &[u8] {
        let range = Self::bytes_height_range();
        &self.buffer()[range]
    }
}

type NodeStorage = dyn Storage<Bytes32, Buffer, Error = StorageError>;

#[derive(Clone)]
struct StorageNode<'storage> {
    storage: &'storage NodeStorage,
    node: Node,
}

impl<'a, 'storage> StorageNode<'storage> {
    pub fn new(storage: &'storage NodeStorage, node: Node) -> Self {
        Self { node, storage }
    }

    pub fn index(&self) -> u64 {
        self.node.index()
    }

    pub fn height(&self) -> u32 {
        self.node.height()
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_leaf()
    }

    pub fn left_child(&'a self) -> Option<Self> {
        if self.node.is_node() {
            let key = self.node.left_child_key();
            let buffer = self.storage.get(key).unwrap().unwrap();
            let node = Node::from_buffer(buffer.as_ref());
            let storage_node = Self {
                storage: self.storage,
                node,
            };
            Some(storage_node)
        } else {
            None
        }
    }

    pub fn right_child(&'a self) -> Option<Self> {
        if self.node.is_node() {
            let key = self.node.right_child_key();
            let buffer = self.storage.get(key).unwrap().unwrap();
            let node = Node::from_buffer(buffer.as_ref());
            let storage_node = Self {
                storage: self.storage,
                node,
            };
            Some(storage_node)
        } else {
            None
        }
    }
}

impl<'storage> crate::common::Node for StorageNode<'storage> {
    fn index(&self) -> u64 {
        StorageNode::index(self)
    }

    fn height(&self) -> u32 {
        StorageNode::height(self)
    }

    fn is_leaf(&self) -> bool {
        StorageNode::is_leaf(self)
    }
}
//
impl<'storage> ParentNode for StorageNode<'storage> {
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
    use crate::common::{IntoPathIterator, StorageMap};

    #[test]
    fn test() {
        let n = Node::create_node(&[0u8; 32], &[1u8; 32], 1);
        let prefix = n.prefix();
        println!("{:?}", prefix);
        let left = n.left_child_key();
        println!("{:?}", left);
        let right = n.right_child_key();
        println!("{:?}", right);
        println!("{:?}", n.value());
    }

    #[test]
    fn test_storage() {
        let mut s = StorageMap::<Bytes32, Buffer>::new();

        let data = [255u8; 32];

        let leaf1 = Node::create_leaf(&[0u8; 32], &data, 0);
        s.insert(&leaf1.value(), leaf1.as_buffer());

        let leaf2 = Node::create_leaf(&[1u8; 32], &data, 2);
        s.insert(&leaf2.value(), leaf2.as_buffer());

        let nn = Node::create_node(&leaf1.value(), &leaf2.value(), 1);
        s.insert(&nn.value(), nn.as_buffer());

        let sn = StorageNode::new(&mut s, nn);

        println!("index {:?}", sn.index());
        println!("height {:?}", sn.height());

        let r = sn.right_child().unwrap();

        println!("{:?}", r.node);
        println!("index {:?}", r.index());
        println!("height {:?}", r.height());

    }

    #[test]
    fn test_iter() {
        let mut s = StorageMap::<Bytes32, Buffer>::new();

        let data = [255u8; 32];

        let leaf1 = Node::create_leaf(&[0u8; 32], &data, 0);
        s.insert(&leaf1.value(), leaf1.as_buffer());

        let leaf2 = Node::create_leaf(&[1u8; 32], &data, 2);
        s.insert(&leaf2.value(), leaf2.as_buffer());

        let nn = Node::create_node(&leaf1.value(), &leaf2.value(), 1);
        s.insert(&nn.value(), nn.as_buffer());

        let leaf = StorageNode::new(&s, leaf1);
        let sn = StorageNode::new(&s, nn);

        let iter = leaf.into_path_iter(&sn);
        for n in iter {
            println!("{:?}", n.node);
        }
    }
}
