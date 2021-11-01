use crate::common::{AsPathIterator, Buffer, Bytes32, Node as NodeTrait, MSB};
use fuel_storage::Storage;

use crate::sparse::{Node, StorageNode};

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
        self.root_node().hash()
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

    #[test]
    fn test_root_returns_empty_sum_with_no_leaves() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let tree = MerkleTree::<StorageError>::new(&mut storage);
        let root = tree.root();
        let expected_root = *empty_sum();
        assert_eq!(root, expected_root);
    }

    ///
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

        // 0x86e5b012af08f415d18599efead53c2714566ecd23f6c439908ab93ab1a0eb40
        let expected_root = [
            0x86_u8, 0xe5_u8, 0xb0_u8, 0x12_u8, 0xaf_u8, 0x08_u8, 0xf4_u8, 0x15_u8, 0xd1_u8,
            0x85_u8, 0x99_u8, 0xef_u8, 0xea_u8, 0xd5_u8, 0x3c_u8, 0x27_u8, 0x14_u8, 0x56_u8,
            0x6e_u8, 0xcd_u8, 0x23_u8, 0xf6_u8, 0xc4_u8, 0x39_u8, 0x90_u8, 0x8a_u8, 0xb9_u8,
            0x3a_u8, 0xb1_u8, 0xa0_u8, 0xeb_u8, 0x40_u8,
        ];
        assert_eq!(root, expected_root);
    }

    #[test]
    fn test_update() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let data = "DATA".as_bytes();
        for i in 0_u32..2 {
            let key = i.to_be_bytes();
            println!("{:?}", key);
            tree.update(&key, data);
        }

        let root = tree.root();
        println!("ROOT {:x?}", root);

        // let expected_root = [
        //     0xdc_u8, 0x05_u8, 0x37_u8, 0x16_u8, 0x74_u8, 0x54_u8, 0x50_u8, 0x9d_u8, 0x36_u8,
        //     0x0e_u8, 0x08_u8, 0x07_u8, 0xb6_u8, 0x73_u8, 0xb0_u8, 0xbd_u8, 0xfd_u8, 0xe7_u8,
        //     0x30_u8, 0xdd_u8, 0x8c_u8, 0xe9_u8, 0x44_u8, 0xa4_u8, 0x3e_u8, 0x39_u8, 0x7e_u8,
        //     0x3a_u8, 0x16_u8, 0xac_u8, 0x32_u8, 0x2b_u8,
        // ];
        // assert_eq!(root, expected_root);
    }
}
