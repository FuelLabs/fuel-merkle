use crate::{
    common::{
        error::DeserializeError,
        path::{ComparablePath, Instruction, Path},
        Bytes32, ChildError, ChildResult, Node as NodeTrait, ParentNode as ParentNodeTrait, Prefix,
    },
    sparse::{
        buffer::{Buffer, ReadView, WriteView, DEFAULT_BUFFER},
        hash::sum,
        merkle_tree::NodesTable,
        zero_sum,
    },
};

use fuel_storage::StorageInspect;

use core::{cmp, fmt};

#[derive(Clone)]
pub(crate) struct Node {
    buffer: Buffer,
}

impl Node {
    pub fn max_height() -> usize {
        Node::key_size_in_bits()
    }

    pub fn create_leaf(key: &Bytes32, data: &[u8]) -> Self {
        let mut buffer = *DEFAULT_BUFFER;
        let mut view = WriteView::new(&mut buffer);
        *view.height_mut() = 0u32;
        *view.prefix_mut() = Prefix::Leaf;
        *view.bytes_lo_mut() = *key;
        *view.bytes_hi_mut() = sum(data);
        Self { buffer }
    }

    pub fn create_node(left_child: &Node, right_child: &Node, height: u32) -> Self {
        let mut buffer = *DEFAULT_BUFFER;
        let mut view = WriteView::new(&mut buffer);
        *view.height_mut() = height;
        *view.prefix_mut() = Prefix::Node;
        *view.bytes_lo_mut() = left_child.hash();
        *view.bytes_hi_mut() = right_child.hash();
        Self { buffer }
    }

    pub fn create_node_on_path(path: &dyn Path, path_node: &Node, side_node: &Node) -> Self {
        if path_node.is_leaf() && side_node.is_leaf() {
            // When joining two leaves, the joined node is found where the paths
            // of the two leaves diverge. The joined node may be a direct parent
            // of the leaves or an ancestor multiple generations above the
            // leaves.
            // N.B.: A leaf can be a placeholder.
            let parent_depth = path_node.common_path_length(side_node);
            let parent_height = (Node::max_height() - parent_depth) as u32;
            match path.get_instruction(parent_depth).unwrap() {
                Instruction::Left => Node::create_node(path_node, side_node, parent_height),
                Instruction::Right => Node::create_node(side_node, path_node, parent_height),
            }
        } else {
            // When joining two nodes, or a node and a leaf, the joined node is
            // the direct parent of the node with the greater height and an
            // ancestor of the node with the lesser height.
            // N.B.: A leaf can be a placeholder.
            let parent_height = cmp::max(path_node.height(), side_node.height()) + 1;
            let parent_depth = Node::max_height() - parent_height as usize;
            match path.get_instruction(parent_depth).unwrap() {
                Instruction::Left => Node::create_node(path_node, side_node, parent_height),
                Instruction::Right => Node::create_node(side_node, path_node, parent_height),
            }
        }
    }

    pub fn create_placeholder() -> Self {
        let buffer = *DEFAULT_BUFFER;
        Self { buffer }
    }

    pub fn common_path_length(&self, other: &Node) -> usize {
        debug_assert!(self.is_leaf());
        debug_assert!(other.is_leaf());

        // If either of the nodes is a placeholder, the common path length is
        // defined to be 0. This is needed to prevent a 0 bit in the
        // placeholder's key from producing an erroneous match with a 0 bit in
        // the leaf's key.
        if self.is_placeholder() || other.is_placeholder() {
            0
        } else {
            self.leaf_key().common_path_length(other.leaf_key())
        }
    }

    pub fn height(&self) -> u32 {
        let view = ReadView::new(&self.buffer);
        *view.height()
    }

    pub fn prefix(&self) -> Prefix {
        let view = ReadView::new(&self.buffer);
        *view.prefix()
    }

    pub fn is_leaf(&self) -> bool {
        self.prefix() == Prefix::Leaf || self.is_placeholder()
    }

    pub fn is_node(&self) -> bool {
        self.prefix() == Prefix::Node
    }

    pub fn leaf_key(&self) -> &Bytes32 {
        assert!(self.is_leaf());
        self.bytes_lo()
    }

    pub fn leaf_data(&self) -> &Bytes32 {
        assert!(self.is_leaf());
        self.bytes_hi()
    }

    pub fn left_child_key(&self) -> &Bytes32 {
        assert!(self.is_node());
        self.bytes_lo()
    }

    pub fn right_child_key(&self) -> &Bytes32 {
        assert!(self.is_node());
        self.bytes_hi()
    }

    pub fn is_placeholder(&self) -> bool {
        *self.bytes_lo() == *zero_sum() && *self.bytes_hi() == *zero_sum()
    }

    pub fn hash(&self) -> Bytes32 {
        if self.is_placeholder() {
            *zero_sum()
        } else {
            let view = ReadView::new(&self.buffer);
            let data = view.bytes_hash();
            sum(data)
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    // PRIVATE

    fn bytes_lo(&self) -> &Bytes32 {
        let view = ReadView::new(&self.buffer);
        let ptr = view.bytes_lo() as *const Bytes32;
        // SAFETY: ptr is guaranteed to point to a valid range of 32 bytes owned
        //         by self.buffer
        unsafe { &*ptr }
    }

    fn bytes_hi(&self) -> &Bytes32 {
        let view = ReadView::new(&self.buffer);
        let ptr = view.bytes_hi() as *const Bytes32;
        // SAFETY: ptr is guaranteed to point to a valid range of 32 bytes owned
        //         by self.buffer
        unsafe { &*ptr }
    }
}

impl TryFrom<Buffer> for Node {
    type Error = DeserializeError;

    fn try_from(buffer: Buffer) -> Result<Self, Self::Error> {
        // Validate the node created from the buffer
        let view = ReadView::new(&buffer);
        let prefix_byte = *view.prefix_byte();
        Prefix::try_from(prefix_byte)?;

        let node = Self { buffer };
        Ok(node)
    }
}

impl NodeTrait for Node {
    type Key = Bytes32;

    fn height(&self) -> u32 {
        Node::height(self)
    }

    fn leaf_key(&self) -> Self::Key {
        *Node::leaf_key(self)
    }

    fn is_leaf(&self) -> bool {
        Node::is_leaf(self)
    }

    fn is_node(&self) -> bool {
        Node::is_node(self)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_node() {
            f.debug_struct("Node (Internal)")
                .field("Height", &self.height())
                .field("Hash", &hex::encode(self.hash()))
                .field("Left child key", &hex::encode(self.left_child_key()))
                .field("Right child key", &hex::encode(self.right_child_key()))
                .finish()
        } else {
            f.debug_struct("Node (Leaf)")
                .field("Height", &self.height())
                .field("Hash", &hex::encode(self.hash()))
                .field("Leaf key", &hex::encode(self.leaf_key()))
                .field("Leaf data", &hex::encode(self.leaf_data()))
                .finish()
        }
    }
}

pub(crate) struct StorageNode<'storage, TableType, StorageType> {
    storage: &'storage StorageType,
    node: Node,
    phantom_table: PhantomData<TableType>,
}

impl<TableType, StorageType> Clone for StorageNode<'_, TableType, StorageType> {
    fn clone(&self) -> Self {
        Self {
            storage: self.storage,
            node: self.node.clone(),
            phantom_table: Default::default(),
        }
    }
}

impl<'s, TableType, StorageType> StorageNode<'s, TableType, StorageType> {
    pub fn new(storage: &'s StorageType, node: Node) -> Self {
        Self {
            node,
            storage,
            phantom_table: Default::default(),
        }
    }
}

impl<TableType, StorageType> StorageNode<'_, TableType, StorageType> {
    pub fn hash(&self) -> Bytes32 {
        self.node.hash()
    }

    pub fn into_node(self) -> Node {
        self.node
    }
}

impl<TableType, StorageType> NodeTrait for StorageNode<'_, TableType, StorageType> {
    type Key = Bytes32;

    fn height(&self) -> u32 {
        self.node.height()
    }

    fn leaf_key(&self) -> Self::Key {
        *self.node.leaf_key()
    }

    fn is_leaf(&self) -> bool {
        self.node.is_leaf()
    }

    fn is_node(&self) -> bool {
        self.node.is_node()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum StorageNodeError<StorageError> {
    #[cfg_attr(feature = "std", error(transparent))]
    StorageError(StorageError),
    #[cfg_attr(feature = "std", error(transparent))]
    DeserializeError(DeserializeError),
}

impl<TableType, StorageType> ParentNodeTrait for StorageNode<'_, TableType, StorageType>
where
    StorageType: StorageInspect<TableType>,
    TableType: Mappable<Key = Bytes32, SetValue = Buffer, GetValue = Buffer>,
    StorageType::Error: fmt::Debug,
{
    type Error = StorageNodeError<StorageType::Error>;

    fn left_child(&self) -> ChildResult<Self> {
        if self.is_leaf() {
            return Err(ChildError::NodeIsLeaf);
        }
        let key = self.node.left_child_key();
        if key == zero_sum() {
            return Ok(Self::new(self.storage, Node::create_placeholder()));
        }
        let buffer = self
            .storage
            .get(key)
            .map_err(StorageNodeError::StorageError)?
            .ok_or(ChildError::ChildNotFound(*key))?;
        Ok(buffer
            .into_owned()
            .try_into()
            .map(|node| Self::new(self.storage, node))
            .map_err(StorageNodeError::DeserializeError)?)
    }

    fn right_child(&self) -> ChildResult<Self> {
        if self.is_leaf() {
            return Err(ChildError::NodeIsLeaf);
        }
        let key = self.node.right_child_key();
        if key == zero_sum() {
            return Ok(Self::new(self.storage, Node::create_placeholder()));
        }
        let buffer = self
            .storage
            .get(key)
            .map_err(StorageNodeError::StorageError)?
            .ok_or(ChildError::ChildNotFound(*key))?;
        Ok(buffer
            .into_owned()
            .try_into()
            .map(|node| Self::new(self.storage, node))
            .map_err(StorageNodeError::DeserializeError)?)
    }
}

impl<TableType, StorageType> fmt::Debug for StorageNode<'_, TableType, StorageType>
where
    StorageType: StorageInspect<TableType>,
    TableType: Mappable<Key = Bytes32, SetValue = Buffer, GetValue = Buffer>,
    StorageType::Error: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_node() {
            f.debug_struct("StorageNode (Internal)")
                .field("Height", &self.height())
                .field("Hash", &hex::encode(self.hash()))
                .field("Left child key", &hex::encode(self.node.left_child_key()))
                .field("Right child key", &hex::encode(self.node.right_child_key()))
                .finish()
        } else {
            f.debug_struct("StorageNode (Leaf)")
                .field("Height", &self.height())
                .field("Hash", &hex::encode(self.hash()))
                .field("Leaf key", &hex::encode(self.node.leaf_key()))
                .field("Leaf data", &hex::encode(self.node.leaf_data()))
                .finish()
        }
    }
}

#[cfg(test)]
mod test_node {
    use crate::{
        common::{error::DeserializeError, Bytes32, Prefix, PrefixError},
        sparse::{hash::sum, zero_sum, Node},
    };

    fn leaf_hash(key: &Bytes32, data: &[u8]) -> Bytes32 {
        let mut buffer = [0; 65];
        buffer[0..1].clone_from_slice(Prefix::Leaf.as_ref());
        buffer[1..33].clone_from_slice(key);
        buffer[33..65].clone_from_slice(&sum(data));
        sum(&buffer)
    }

    #[test]
    fn test_create_leaf_returns_a_valid_leaf() {
        let leaf = Node::create_leaf(&sum(b"LEAF"), &[1u8; 32]);
        assert_eq!(leaf.is_leaf(), true);
        assert_eq!(leaf.is_node(), false);
        assert_eq!(leaf.height(), 0);
        assert_eq!(leaf.prefix(), Prefix::Leaf);
        assert_eq!(*leaf.leaf_key(), sum(b"LEAF"));
        assert_eq!(*leaf.leaf_data(), sum(&[1u8; 32]));
    }

    #[test]
    fn test_create_node_returns_a_valid_node() {
        let left_child = Node::create_leaf(&sum(b"LEFT"), &[1u8; 32]);
        let right_child = Node::create_leaf(&sum(b"RIGHT"), &[1u8; 32]);
        let node = Node::create_node(&left_child, &right_child, 1);
        assert_eq!(node.is_leaf(), false);
        assert_eq!(node.is_node(), true);
        assert_eq!(node.height(), 1);
        assert_eq!(node.prefix(), Prefix::Node);
        assert_eq!(*node.left_child_key(), leaf_hash(&sum(b"LEFT"), &[1u8; 32]));
        assert_eq!(
            *node.right_child_key(),
            leaf_hash(&sum(b"RIGHT"), &[1u8; 32])
        );
    }

    #[test]
    fn test_create_placeholder_returns_a_placeholder_node() {
        let node = Node::create_placeholder();
        assert_eq!(node.is_placeholder(), true);
        assert_eq!(node.hash(), *zero_sum());
    }

    #[test]
    fn test_create_leaf_from_buffer_returns_a_valid_leaf() {
        let mut buffer = [0u8; 69];
        buffer[0..4].clone_from_slice(&0_u32.to_ne_bytes());
        buffer[4..5].clone_from_slice(Prefix::Leaf.as_ref());
        buffer[5..37].clone_from_slice(&[1u8; 32]);
        buffer[37..69].clone_from_slice(&[1u8; 32]);

        let node: Node = buffer.try_into().unwrap();
        assert_eq!(node.is_leaf(), true);
        assert_eq!(node.is_node(), false);
        assert_eq!(node.height(), 0);
        assert_eq!(node.prefix(), Prefix::Leaf);
        assert_eq!(*node.leaf_key(), [1u8; 32]);
        assert_eq!(*node.leaf_data(), [1u8; 32]);
    }

    #[test]
    fn test_create_node_from_buffer_returns_a_valid_node() {
        let mut buffer = [0u8; 69];
        buffer[0..4].clone_from_slice(&256_u32.to_ne_bytes());
        buffer[4..5].clone_from_slice(Prefix::Node.as_ref());
        buffer[5..37].clone_from_slice(&[1u8; 32]);
        buffer[37..69].clone_from_slice(&[1u8; 32]);

        let node: Node = buffer.try_into().unwrap();
        assert_eq!(node.is_leaf(), false);
        assert_eq!(node.is_node(), true);
        assert_eq!(node.height(), 256);
        assert_eq!(node.prefix(), Prefix::Node);
        assert_eq!(*node.left_child_key(), [1u8; 32]);
        assert_eq!(*node.right_child_key(), [1u8; 32]);
    }

    #[test]
    fn test_create_from_buffer_returns_deserialize_error_if_invalid_prefix() {
        let mut buffer = [0u8; 69];
        buffer[0..4].clone_from_slice(&0_u32.to_ne_bytes());
        buffer[4..5].clone_from_slice(&[0x02]);
        buffer[5..37].clone_from_slice(&[1u8; 32]);
        buffer[37..69].clone_from_slice(&[1u8; 32]);

        // Should return Error; prefix 0x02 is does not represent a node or leaf
        let err = Node::try_from(buffer).expect_err("Expected try_from() to be Error; got OK");
        assert!(matches!(
            err,
            DeserializeError::PrefixError(PrefixError::InvalidPrefix(0x02))
        ));
    }

    /// For leaf node `node` of leaf data `d` with key `k`:
    /// ```node.buffer = (0x00, k, h(serialize(d)))```
    #[test]
    fn test_leaf_buffer_returns_expected_buffer() {
        let mut expected_buffer = [0u8; 69];
        expected_buffer[0..4].clone_from_slice(&0_u32.to_ne_bytes());
        expected_buffer[4..5].clone_from_slice(Prefix::Leaf.as_ref());
        expected_buffer[5..37].clone_from_slice(&sum(b"LEAF"));
        expected_buffer[37..69].clone_from_slice(&sum(&[1u8; 32]));

        let leaf = Node::create_leaf(&sum(b"LEAF"), &[1u8; 32]);
        let buffer = leaf.buffer();

        assert_eq!(*buffer, expected_buffer);
    }

    /// For internal node `node` with children `l` and `r`:
    /// ```node.buffer = (0x01, l.v, r.v)```
    #[test]
    fn test_node_buffer_returns_expected_buffer() {
        let mut expected_buffer = [0u8; 69];
        expected_buffer[0..4].clone_from_slice(&1_u32.to_ne_bytes());
        expected_buffer[4..5].clone_from_slice(Prefix::Node.as_ref());
        expected_buffer[5..37].clone_from_slice(&leaf_hash(&sum(b"LEFT"), &[1u8; 32]));
        expected_buffer[37..69].clone_from_slice(&leaf_hash(&sum(b"RIGHT"), &[1u8; 32]));

        let left_child = Node::create_leaf(&sum(b"LEFT"), &[1u8; 32]);
        let right_child = Node::create_leaf(&sum(b"RIGHT"), &[1u8; 32]);
        let node = Node::create_node(&left_child, &right_child, 1);
        let buffer = node.buffer();

        assert_eq!(*buffer, expected_buffer);
    }

    /// For leaf node `node` of leaf data `d` with key `k`:
    /// ```node.v = h(0x00, k, h(serialize(d)))```
    #[test]
    fn test_leaf_hash_returns_expected_hash_value() {
        let mut expected_buffer = [0u8; 65];
        expected_buffer[0..1].clone_from_slice(Prefix::Leaf.as_ref());
        expected_buffer[1..33].clone_from_slice(&sum(b"LEAF"));
        expected_buffer[33..65].clone_from_slice(&sum(&[1u8; 32]));
        let expected_value = sum(&expected_buffer);

        let node = Node::create_leaf(&sum(b"LEAF"), &[1u8; 32]);
        let value = node.hash();

        assert_eq!(value, expected_value);
    }

    /// For internal node `node` with children `l` and `r`:
    /// ```node.v = h(0x01, l.v, r.v)```
    #[test]
    fn test_node_hash_returns_expected_hash_value() {
        let mut expected_buffer = [0u8; 65];
        expected_buffer[0..1].clone_from_slice(Prefix::Node.as_ref());
        expected_buffer[1..33].clone_from_slice(&leaf_hash(&sum(b"LEFT"), &[1u8; 32]));
        expected_buffer[33..65].clone_from_slice(&leaf_hash(&sum(b"RIGHT"), &[1u8; 32]));
        let expected_value = sum(&expected_buffer);

        let left_child = Node::create_leaf(&sum(b"LEFT"), &[1u8; 32]);
        let right_child = Node::create_leaf(&sum(b"RIGHT"), &[1u8; 32]);
        let node = Node::create_node(&left_child, &right_child, 1);
        let value = node.hash();

        assert_eq!(value, expected_value);
    }
}

#[cfg(test)]
mod test_storage_node {
    use crate::{
        common::{
            error::DeserializeError, Bytes32, ChildError, ParentNode, PrefixError, StorageMap,
        },
        sparse::{
            buffer::BUFFER_SIZE, hash::sum, merkle_tree::NodesTable, node::StorageNodeError, Node,
            StorageNode,
        },
    };

    use fuel_storage::{Mappable, StorageMutate};

    pub struct NodesTable;

    impl Mappable for NodesTable {
        type Key = Bytes32;
        type SetValue = Buffer;
        type GetValue = Self::SetValue;
    }

    #[test]
    fn test_node_left_child_returns_the_left_child() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let _ = s.insert(&leaf_0.hash(), leaf_0.buffer());

        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let _ = s.insert(&leaf_1.hash(), leaf_1.buffer());

        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);
        let _ = s.insert(&node_0.hash(), node_0.buffer());

        let storage_node = StorageNode::new(&s, node_0);
        let child = storage_node.left_child().unwrap();

        assert_eq!(child.hash(), leaf_0.hash());
    }

    #[test]
    fn test_node_right_child_returns_the_right_child() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let _ = s.insert(&leaf_0.hash(), leaf_0.buffer());

        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let _ = s.insert(&leaf_1.hash(), leaf_1.buffer());

        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);
        let _ = s.insert(&node_0.hash(), node_0.buffer());

        let storage_node = StorageNode::new(&s, node_0);
        let child = storage_node.right_child().unwrap();

        assert_eq!(child.hash(), leaf_1.hash());
    }

    #[test]
    fn test_node_left_child_returns_placeholder_when_key_is_zero_sum() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let _ = s.insert(&leaf.hash(), leaf.buffer());

        let node_0 = Node::create_node(&Node::create_placeholder(), &leaf, 1);
        let _ = s.insert(&node_0.hash(), node_0.buffer());

        let storage_node = StorageNode::new(&s, node_0);
        let child = storage_node.left_child().unwrap();

        assert!(child.node.is_placeholder());
    }

    #[test]
    fn test_node_right_child_returns_placeholder_when_key_is_zero_sum() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let _ = s.insert(&leaf.hash(), leaf.buffer());

        let node_0 = Node::create_node(&leaf, &Node::create_placeholder(), 1);
        let _ = s.insert(&node_0.hash(), node_0.buffer());

        let storage_node = StorageNode::new(&s, node_0);
        let child = storage_node.right_child().unwrap();

        assert!(child.node.is_placeholder());
    }

    #[test]
    fn test_node_left_child_returns_error_when_node_is_leaf() {
        let s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let storage_node = StorageNode::new(&s, leaf_0);
        let err = storage_node
            .left_child()
            .expect_err("Expected left_child() to return Error; got OK");

        assert!(matches!(err, ChildError::NodeIsLeaf));
    }

    #[test]
    fn test_node_right_child_returns_error_when_node_is_leaf() {
        let s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let storage_node = StorageNode::new(&s, leaf_0);
        let err = storage_node
            .right_child()
            .expect_err("Expected right_child() to return Error; got OK");

        assert!(matches!(err, ChildError::NodeIsLeaf));
    }

    #[test]
    fn test_node_left_child_returns_error_when_key_is_not_found() {
        let s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[0u8; 32]);
        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);

        let storage_node = StorageNode::new(&s, node_0);
        let err = storage_node
            .left_child()
            .expect_err("Expected left_child() to return Error; got Ok");

        let key = *storage_node.into_node().left_child_key();
        assert!(matches!(
            err,
            ChildError::ChildNotFound(k) if k == key
        ));
    }

    #[test]
    fn test_node_right_child_returns_error_when_key_is_not_found() {
        let s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);

        let storage_node = StorageNode::new(&s, node_0);
        let err = storage_node
            .right_child()
            .expect_err("Expected right_child() to return Error; got Ok");

        let key = *storage_node.into_node().right_child_key();
        assert!(matches!(
            err,
            ChildError::ChildNotFound(k) if k == key
        ));
    }

    #[test]
    fn test_node_left_child_returns_deserialize_error_when_buffer_is_invalid() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let _ = s.insert(&leaf_0.hash(), &[255; BUFFER_SIZE]);
        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);

        let storage_node = StorageNode::new(&s, node_0);
        let err = storage_node
            .left_child()
            .expect_err("Expected left_child() to be Error; got Ok");

        assert!(matches!(
            err,
            ChildError::Error(StorageNodeError::DeserializeError(
                DeserializeError::PrefixError(PrefixError::InvalidPrefix(255))
            ))
        ));
    }

    #[test]
    fn test_node_right_child_returns_deserialize_error_when_buffer_is_invalid() {
        let mut s = StorageMap::<NodesTable>::new();

        let leaf_0 = Node::create_leaf(&sum(b"Hello World"), &[1u8; 32]);
        let leaf_1 = Node::create_leaf(&sum(b"Goodbye World"), &[1u8; 32]);
        let _ = s.insert(&leaf_1.hash(), &[255; BUFFER_SIZE]);
        let node_0 = Node::create_node(&leaf_0, &leaf_1, 1);

        let storage_node = StorageNode::new(&s, node_0);
        let err = storage_node
            .right_child()
            .expect_err("Expected right_child() to be Error; got Ok");

        assert!(matches!(
            err,
            ChildError::Error(StorageNodeError::DeserializeError(
                DeserializeError::PrefixError(PrefixError::InvalidPrefix(255))
            ))
        ));
    }
}
