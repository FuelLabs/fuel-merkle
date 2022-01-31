use crate::common::{AsPathIterator, Bytes32, Msb, Node as NodeTrait};
use fuel_storage::Storage;

use crate::sparse::hash::sum;
use crate::sparse::{zero_sum, Buffer, Node, StorageNode};

pub struct MerkleTree<'storage, StorageError> {
    root_node: Node,
    storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>,
}

impl<'a, 'storage, StorageError> MerkleTree<'storage, StorageError>
where
    StorageError: std::error::Error + Clone + 'static,
{
    pub fn new(storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>) -> Self {
        let root_node = Node::create_placeholder();

        let _ = storage.insert(&root_node.hash(), root_node.as_buffer());

        Self { root_node, storage }
    }

    pub fn update(&'a mut self, key: &[u8], data: &[u8]) -> Result<(), Box<dyn std::error::Error>>{
        if data.is_empty() {
            // If the data is empty, this signifies a delete operation for the given key.
            self.delete(key);
            return Ok(());
        }

        let leaf_node = Node::create_leaf(key, data);
        self.storage
            .insert(&leaf_node.hash(), leaf_node.as_buffer())?;
        self.storage
            .insert(&leaf_node.leaf_key(), leaf_node.as_buffer())?;
        let (path_nodes, side_nodes): (Vec<Node>, Vec<Node>) = self.path_set(leaf_node.clone());
        self.update_with_path_set(&leaf_node, path_nodes.as_slice(), side_nodes.as_slice());

        Ok(())
    }

    pub fn delete(&'a mut self, key: &[u8]) {
        if self.root() == *zero_sum() {
            // The zero root signifies that all leaves are empty, including the given key.
            return;
        }

        let leaf_key = sum(key);
        if let Some(buffer) = self.storage.get(&leaf_key).unwrap() {
            let leaf_node = Node::from_buffer(*buffer);
            let (path_nodes, side_nodes): (Vec<Node>, Vec<Node>) = self.path_set(leaf_node.clone());
            self.delete_with_path_set(&leaf_node, path_nodes.as_slice(), side_nodes.as_slice());
        }
    }

    pub fn root(&'a self) -> Bytes32 {
        self.root_node().hash()
    }

    // PRIVATE

    fn depth(&'a self) -> usize {
        Node::key_size_in_bits()
    }

    fn root_node(&'a self) -> &Node {
        &self.root_node
    }

    fn insert(&'a mut self, node: &Node) {
        let _ = self.storage.insert(&node.hash(), node.as_buffer());
    }

    fn path_set(&'a self, leaf_node: Node) -> (Vec<Node>, Vec<Node>) {
        let root_node = self.root_node().clone();
        println!("ROOT: {:?}", root_node);
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
        requested_leaf_node: &Node,
        path_nodes: &[Node],
        side_nodes: &[Node],
    ) {
        let actual_leaf_node = &path_nodes[0];
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
            // 1. Determine the depth of the common radix ancestor relative to the root, using the
            //    common prefix count (CPC).
            // 2. Create a parent for the leaf node we traversed to and the leaf we are updating.
            //    Invert the depth to get the parent's height.
            // 3. Check the requested leaf's bit at the CPC to determine it's orientation relative
            //    to the radix ancestor. If the requested leaf is on the left of the radix ancestor,
            //    it is the parent's left child; otherwise, it is the parent's right child.
            let requested_leaf_key = requested_leaf_node.leaf_key();
            if requested_leaf_key
                .get_bit_at_index_from_msb(common_prefix_count)
                .unwrap()
                == 1
            {
                current_node = Node::create_node(&actual_leaf_node, &current_node);
            } else {
                current_node = Node::create_node(&current_node, &actual_leaf_node);
            }
            current_node.set_height((self.depth() - common_prefix_count) as u32);
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
            if requested_leaf_key
                .get_bit_at_index_from_msb(self.depth() - 1 - i)
                .unwrap()
                == 1
            {
                current_node = Node::create_node(&side_node, &current_node);
            } else {
                current_node = Node::create_node(&current_node, &side_node);
            }
            self.insert(&current_node);
        }

        self.root_node = current_node;
    }

    fn delete_with_path_set(
        &'a mut self,
        requested_leaf_node: &Node,
        path_nodes: &[Node],
        side_nodes: &[Node],
    ) {
        for node in path_nodes {
            let _ = self.storage.remove(&node.hash());
        }

        let mut non_placeholder_reached = false;
        let mut current_node = Node::create_placeholder();
        let n = side_nodes.len();
        for i in 0..n {
            let side_node = &side_nodes[i];
            if current_node.is_placeholder() {
                if side_node.is_leaf() {
                    current_node = side_node.clone();
                    continue;
                } else {
                    non_placeholder_reached = true;
                }
            }

            if !non_placeholder_reached && side_node.is_placeholder() {
                continue;
            } else if !non_placeholder_reached {
                non_placeholder_reached = true;
            }

            let requested_leaf_key = requested_leaf_node.leaf_key();
            if requested_leaf_key
                .get_bit_at_index_from_msb(self.depth() - 1 - i)
                .unwrap()
                == 1
            {
                current_node = Node::create_node(&side_node, &current_node);
            } else {
                current_node = Node::create_node(&current_node, &side_node);
            }
            self.insert(&current_node);
        }

        self.root_node = current_node;
    }
}

#[cfg(test)]
mod test {
    use crate::common::{Bytes32, StorageError, StorageMap};
    use crate::sparse::{Buffer, MerkleTree};
    use hex;

    #[test]
    fn test_root_returns_empty_sum_with_no_leaves() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let tree = MerkleTree::<StorageError>::new(&mut storage);
        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..1 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
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

    #[test]
    fn test_update_empty() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let key = 0_u32.to_be_bytes();
        let data = [0_u8; 0];
        tree.update(&key, &data);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_delete_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let key = 1_u32.to_be_bytes();
        let data = "DATA".as_bytes();
        tree.update(&key, data);
        tree.delete(&key);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_update_empty() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        let key = 0_u32.to_be_bytes();
        let data = "DATA".as_bytes();
        tree.update(&key, data);
        tree.update(&key, &[0; 0]);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2_delete_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..2 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let key = 0_u32.to_be_bytes();
        tree.delete(&key);

        let root = tree.root();
        let expected_root = "d7cb6616832899ac111a852ca8df2d63a1cdb36cb84651ffde72e264506a456f";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_delete_non_existent_key_does_not_change_root() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for i in 0_u32..5 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            tree.update(&key, data);
        }

        let key = 1024_u32.to_be_bytes();
        tree.delete(&key);

        let root = tree.root();
        let expected_root = "108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b";
        assert_eq!(hex::encode(root), expected_root);
    }
}
