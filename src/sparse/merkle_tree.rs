use crate::common::{AsPathIterator, Buffer, Bytes32, Node as NodeTrait, MSB};
use fuel_storage::Storage;

use crate::sparse::{empty_sum, Node, StorageNode};

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("Error")]
    Error(),
}

pub struct MerkleTree<'storage, StorageError> {
    root_node: Node,
    storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>,
}

impl<'a, 'storage, StorageError> MerkleTree<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    pub fn new(storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>) -> Self {
        let root_node = Node::create_placeholder();
        let _ = storage.insert(&root_node.hash(), root_node.as_buffer());

        Self { root_node, storage }
    }

    pub fn update(&'a mut self, key: &[u8], data: &[u8]) {
        let leaf_node = Node::create_leaf(key, data);
        self.update_for_root(leaf_node);
    }

    pub fn root(&self) -> Bytes32 {
        if self.root_node.is_placeholder() {
            *empty_sum()
        } else {
            self.root_node().hash()
        }
    }

    // PRIVATE

    fn depth(&self) -> usize {
        Node::key_size_in_bits()
    }

    fn root_node(&self) -> &Node {
        &self.root_node
    }

    fn insert(&'a mut self, node: &Node) {
        let _ = self.storage.insert(&node.hash(), node.as_buffer());
    }

    fn update_for_root(&mut self, leaf_node: Node) {
        let (path_nodes, side_nodes): (Vec<Node>, Vec<Node>) = self.path_set(leaf_node.clone());
        if leaf_node.is_placeholder() {
            self.delete_with_path_set(path_nodes.as_slice(), side_nodes.as_slice());
        } else {
            self.insert(&leaf_node);
            self.update_with_path_set(leaf_node, path_nodes.as_slice(), side_nodes.as_slice());
        }
    }

    fn path_set(&self, leaf_node: Node) -> (Vec<Node>, Vec<Node>) {
        let root_node = self.root_node.clone();
        let root_storage_node = StorageNode::<StorageError>::new(self.storage, root_node);
        let leaf_storage_node = StorageNode::<StorageError>::new(self.storage, leaf_node);
        let (mut path_nodes, mut side_nodes): (Vec<Node>, Vec<Node>) = root_storage_node
            .as_path_iter(&leaf_storage_node)
            .map(|(node, side_node)| (node.into_node(), side_node.into_node()))
            .unzip();
        path_nodes.reverse();
        side_nodes.reverse();
        side_nodes.pop(); // The last element in the side nodes list is the root; remove it.

        (path_nodes, side_nodes)
    }

    fn update_with_path_set(
        &'a mut self,
        requested_leaf_node: Node,
        path_nodes: &[Node],
        side_nodes: &[Node],
    ) {
        let actual_leaf_node = path_nodes[0].clone();
        let mut current_node = requested_leaf_node.clone();

        let common_prefix_count = {
            if actual_leaf_node.is_placeholder() {
                self.depth()
            } else {
                let actual_leaf_key = actual_leaf_node.leaf_key();
                let requested_leaf_key = requested_leaf_node.leaf_key();
                actual_leaf_key.common_prefix_count(requested_leaf_key)
            }
        };
        if common_prefix_count != self.depth() {
            let requested_leaf_key = requested_leaf_node.leaf_key();
            if requested_leaf_key.get_bit_at_index_from_msb(common_prefix_count) == 1 {
                current_node = Node::create_node(&actual_leaf_node.hash(), &current_node.hash());
            } else {
                current_node = Node::create_node(&current_node.hash(), &actual_leaf_node.hash());
            }
            self.insert(&current_node);
        }

        let offset_side_nodes = self.depth() - side_nodes.len();
        for i in 0..self.depth() {
            let side_node = {
                if i < offset_side_nodes {
                    if common_prefix_count != self.depth()
                        && common_prefix_count > self.depth() - 1 - i
                    {
                        Node::create_placeholder()
                    } else {
                        continue;
                    }
                } else {
                    side_nodes[i - offset_side_nodes].clone()
                }
            };

            let requested_leaf_key = requested_leaf_node.leaf_key();
            if requested_leaf_key.get_bit_at_index_from_msb(self.depth() - 1 - i) == 1 {
                current_node = Node::create_node(&side_node.hash(), &current_node.hash());
            } else {
                current_node = Node::create_node(&current_node.hash(), &side_node.hash());
            }
            self.insert(&current_node);
        }

        self.root_node = current_node;
    }

    fn delete_with_path_set(&'a self, _path_nodes: &[Node], _side_nodes: &[Node]) -> Bytes32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Buffer, Bytes32, StorageError, StorageMap};
    use crate::sparse::{empty_sum, MerkleTree};
    use hex;

    #[test]
    fn test_root_returns_empty_sum_with_no_leaves() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let tree = MerkleTree::<StorageError>::new(&mut storage);
        let root = tree.root();
        let expected_root = *empty_sum();
        assert_eq!(root, expected_root);
    }

    /// ```text
    /// 32:                o
    ///                   / \
    ///                  /   \
    ///                 /     |
    ///               ...     0
    ///              /   \
    ///             /     \
    /// 2:         o       |
    ///           / \      0
    ///          /   \
    ///         /     \
    /// 1:     o       o
    ///        |      / \
    /// 0:     0     o   o
    ///              |   |
    ///              D   0
    /// ```
    #[test]
    fn test_update_one() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let key = "testKey1".as_bytes();
        let data = "testValue1".as_bytes();
        tree.update(key, data);

        let root = tree.root();
        let expected_root = "86e5b012af08f415d18599efead53c2714566ecd23f6c439908ab93ab1a0eb40";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..2 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_3() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..3 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_5() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..5 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_100() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..100 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "82bf747d455a55e2f7044a03536fc43f1f55d43b855e72c0110c986707a23e4d";
        assert_eq!(hex::encode(root), expected_root);
    }
}
