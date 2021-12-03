use crate::common::Bytes32;
use core::fmt;
use fuel_storage::Storage;

#[derive(Clone)]
pub struct Node {
    height: u32,
    hash: Bytes32,
    fee: u32,
    left_child_key: Option<Bytes32>,
    right_child_key: Option<Bytes32>,
}

impl Node {
    pub fn new(height: u32, hash: Bytes32, fee: u32) -> Self {
        Self {
            height,
            hash,
            fee,
            left_child_key: None,
            right_child_key: None,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn hash(&self) -> Bytes32 {
        self.hash.clone()
    }

    pub fn fee(&self) -> u32 {
        self.fee
    }

    pub fn left_child_key(&self) -> Option<Bytes32> {
        self.left_child_key.clone()
    }

    pub fn right_child_key(&self) -> Option<Bytes32> {
        self.right_child_key.clone()
    }

    pub fn set_left_child_key(&mut self, key: Option<Bytes32>) {
        self.left_child_key = key;
    }

    pub fn set_right_child_key(&mut self, key: Option<Bytes32>) {
        self.right_child_key = key;
    }

    pub fn is_leaf(&self) -> bool {
        self.height == 0
    }

    pub fn is_node(&self) -> bool {
        !self.is_leaf()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_node() {
            f.debug_struct("Node (Internal)")
                .field("Hash", &hex::encode(self.hash()))
                .field("Fee", &self.fee)
                .field(
                    "Left child key",
                    &hex::encode(&self.left_child_key().unwrap()),
                )
                .field(
                    "Right child key",
                    &hex::encode(&self.right_child_key().unwrap()),
                )
                .finish()
        } else {
            f.debug_struct("Node (Leaf)")
                .field("Hash", &hex::encode(self.hash()))
                .field("Fee", &self.fee)
                .field("Key", &hex::encode(self.hash()))
                .finish()
        }
    }
}

type NodeStorage<'storage, StorageError> =
    dyn 'storage + Storage<Bytes32, Node, Error = StorageError>;

#[derive(Clone)]
pub(crate) struct StorageNode<'storage, StorageError> {
    storage: &'storage NodeStorage<'storage, StorageError>,
    node: Node,
}

impl<'a, 'storage, StorageError> StorageNode<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    pub fn new(storage: &'storage NodeStorage<'storage, StorageError>, node: Node) -> Self {
        Self { node, storage }
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_leaf()
    }

    pub fn is_node(&self) -> bool {
        self.node.is_node()
    }

    pub fn leaf_key(&self) -> Bytes32 {
        self.node.hash()
    }

    pub fn left_child(&self) -> Option<Self> {
        let key = self.node.left_child_key().unwrap();
        let node = self.storage.get(&key).unwrap();
        node.map(|n| Self::new(self.storage, n.into_owned()))
    }

    pub fn right_child(&self) -> Option<Self> {
        let key = self.node.right_child_key().unwrap();
        let node = self.storage.get(&key).unwrap();
        node.map(|n| Self::new(self.storage, n.into_owned()))
    }

    pub fn into_node(self) -> Node {
        self.node
    }
}

impl<'storage, StorageError> crate::common::Node for StorageNode<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    type Key = Bytes32;

    fn leaf_key(&self) -> Self::Key {
        StorageNode::leaf_key(self)
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