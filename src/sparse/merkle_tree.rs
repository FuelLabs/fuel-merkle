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
        let _ = storage.insert(&root_node.value(), root_node.as_buffer());

        Self { root_node, storage }
    }

    pub fn update(&'a mut self, key: &[u8], data: &[u8]) {
        let leaf_node = Node::create_leaf(key, data);
        self.update_for_root(leaf_node);
    }

    pub fn root(&self) -> Bytes32 {
        self.root_node().value()
    }

    // PRIVATE

    fn depth(&self) -> usize {
        Node::key_size_in_bits()
    }

    fn root_node(&self) -> &Node {
        &self.root_node
    }

    fn insert(&'a mut self, node: &Node) {
        let _ = self.storage.insert(&node.value(), node.as_buffer());
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
        side_nodes.pop(); // The last element in the side nodes list is the root; remove it

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
                current_node = Node::create_node(&actual_leaf_node.value(), &current_node.value());
            } else {
                current_node = Node::create_node(&current_node.value(), &actual_leaf_node.value());
            }
            self.insert(&current_node);
        }

        let offset_side_nodes = self.depth() - side_nodes.len();
        for i in 0..self.depth() {
            let mut side_node = Node::create_placeholder();
            if i < offset_side_nodes {
                let a = common_prefix_count != self.depth();
                let b = common_prefix_count > self.depth() - 1 - i;
                if a && b {
                    side_node = Node::create_placeholder();
                } else {
                    continue;
                }
            } else {
                side_node = side_nodes[i - offset_side_nodes].clone();
            }

            let requested_leaf_key = requested_leaf_node.leaf_key();
            if requested_leaf_key.get_bit_at_index_from_msb(self.depth() - 1 - i) == 1 {
                current_node = Node::create_node(&side_node.value(), &current_node.value());
            } else {
                current_node = Node::create_node(&current_node.value(), &side_node.value());
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
    use crate::sparse::hash::sum;
    use crate::sparse::{empty_sum, MerkleTree};

    #[test]
    fn test_root_returns_empty_sum_with_no_leaves() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let tree = MerkleTree::<StorageError>::new(&mut storage);
        let root = tree.root();
        let expected_root = *empty_sum();
        assert_eq!(root, expected_root);
    }

    // #[test]
    // fn test_update_one() {
    //
    //      32:              (2^32)-1
    //                        /  \
    //                       /    \
    //        (2^32)-(2^31)-1      (2^32)+(2^31)-1
    //                     /       |
    //                   ...       0
    //                   /
    //                  /
    //      2:        03
    //               /  \
    //              /    \
    //      1:     01      05
    //            /  \     |
    //      0:   00  02    0
    //           |   |
    //           D   0
    //
    //     let mut storage = StorageMap::<Bytes32, Buffer>::new();
    //     let mut tree = MerkleTree::<StorageError>::new(&mut storage);
    //
    //     let key = 0_u32.to_be_bytes();
    //     let data = 42_u32.to_be_bytes();
    //     tree.update(&key, &data);
    //
    //     let root = tree.root();
    //     println!("{:x?}", root);
    // }

    #[test]
    fn test_update() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let data = 42_u32.to_be_bytes();
        for i in 0_u32..100 {
            let key = i.to_be_bytes();
            let sum_key = sum(&key);
            tree.update(&sum_key, &data);
        }

        let root = tree.root();
        println!("{:x?}", root);
        let expected_root = [
            0xdc_u8, 0x05_u8, 0x37_u8, 0x16_u8, 0x74_u8, 0x54_u8, 0x50_u8, 0x9d_u8, 0x36_u8,
            0x0e_u8, 0x08_u8, 0x07_u8, 0xb6_u8, 0x73_u8, 0xb0_u8, 0xbd_u8, 0xfd_u8, 0xe7_u8,
            0x30_u8, 0xdd_u8, 0x8c_u8, 0xe9_u8, 0x44_u8, 0xa4_u8, 0x3e_u8, 0x39_u8, 0x7e_u8,
            0x3a_u8, 0x16_u8, 0xac_u8, 0x32_u8, 0x2b_u8,
        ];
        println!("{:x?}", expected_root);
    }
}
