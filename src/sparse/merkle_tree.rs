use crate::common::{AsPathIterator, Bytes32, Node as NodeTrait};
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
    pub fn new(
        storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let root_node = Node::create_placeholder();

        storage.insert(&root_node.hash(), root_node.as_buffer())?;

        Ok(Self { root_node, storage })
    }

    pub fn update(&'a mut self, key: &[u8], data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if data.is_empty() {
            // If the data is empty, this signifies a delete operation for the given key.
            self.delete(key)?;
            return Ok(());
        }

        let leaf_node = Node::create_leaf(key, data);

        // println!();
        // println!("Updating...");
        // println!("Path: {}", hex::encode(leaf_node.leaf_key()));
        // println!("Inserting (req): {:?}", leaf_node);

        self.storage
            .insert(&leaf_node.hash(), leaf_node.as_buffer())?;
        self.storage
            .insert(&leaf_node.leaf_key(), leaf_node.as_buffer())?;

        // if self.root_node().is_placeholder() {
        //     self.set_root_node(leaf_node);
        // } else {
        let (path_nodes, side_nodes): (Vec<Node>, Vec<Node>) = self.path_set(leaf_node.clone());
        self.update_with_path_set(&leaf_node, path_nodes.as_slice(), side_nodes.as_slice())?;
        // }

        Ok(())
    }

    pub fn delete(&'a mut self, key: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.root() == *zero_sum() {
            // The zero root signifies that all leaves are empty, including the given key.
            return Ok(());
        }

        let leaf_key = sum(key);
        if let Some(buffer) = self.storage.get(&leaf_key).unwrap() {
            let leaf_node = Node::from_buffer(*buffer);
            let (path_nodes, side_nodes): (Vec<Node>, Vec<Node>) = self.path_set(leaf_node.clone());
            self.delete_with_path_set(&leaf_node, path_nodes.as_slice(), side_nodes.as_slice())?;
        }

        Ok(())
    }

    pub fn root(&'a self) -> Bytes32 {
        self.root_node().hash()
    }

    // PRIVATE

    fn max_height(&'a self) -> usize {
        Node::key_size_in_bits()
    }

    fn root_node(&'a self) -> &Node {
        &self.root_node
    }

    fn set_root_node(&'a mut self, node: Node) {
        assert!(node.is_leaf() || node.height() == self.max_height() as u32);
        self.root_node = node;
    }

    fn path_set(&'a self, leaf_node: Node) -> (Vec<Node>, Vec<Node>) {
        let root_node = self.root_node().clone();
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let actual_leaf_node = &path_nodes[0];
        let path = requested_leaf_node.leaf_key();
        let mut current_node = requested_leaf_node.clone();

        // Merge leaves
        if !actual_leaf_node.is_placeholder() {
            current_node = Node::create_node_on_path(path, requested_leaf_node, actual_leaf_node);
            self.storage
                .insert(&current_node.hash(), current_node.as_buffer())?;
        }

        // Merge placeholders
        let ancestor_depth = requested_leaf_node.common_path_length(actual_leaf_node);
        let stale_depth = std::cmp::max(side_nodes.len(), ancestor_depth);
        let placeholders_count = stale_depth - side_nodes.len();
        let placeholders = std::iter::repeat(Node::create_placeholder()).take(placeholders_count);
        for placeholder in placeholders {
            current_node = Node::create_node_on_path(path, &current_node, &placeholder);
            self.storage
                .insert(&current_node.hash(), current_node.as_buffer())?;
        }

        // Merge side nodes
        for side_node in side_nodes {
            current_node = Node::create_node_on_path(path, &current_node, &side_node);
            self.storage
                .insert(&current_node.hash(), current_node.as_buffer())?;
        }

        self.set_root_node(current_node);

        Ok(())
    }

    fn delete_with_path_set(
        &'a mut self,
        requested_leaf_node: &Node,
        path_nodes: &[Node],
        side_nodes: &[Node],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for node in path_nodes {
            self.storage.remove(&node.hash())?;
        }

        if side_nodes.is_empty() {
            self.set_root_node(Node::create_placeholder());
            return Ok(());
        }

        let path = requested_leaf_node.leaf_key();
        let first_side_node = side_nodes.first().unwrap(); // Safety: side_nodes is not empty
        let mut side_nodes_iter = side_nodes.iter();

        // The deleted leaf is replaced by a placeholder.
        let mut current_node = Node::create_placeholder();

        // If the first side node is a leaf, it means the ancestor node is now parent to a
        // placeholder (the deleted leaf node) and a leaf node (the first side node). We can
        // immediately discard the ancestor node from further calculation and attach the orphaned
        // leaf node to its next ancestor. Any subsequent ancestor nodes composed of this leaf node
        // and a placeholder must be similarly discarded from further calculation. We then create a
        // valid ancestor node for the orphaned leaf node by joining it with the earliest
        // non-placeholder side node.
        if first_side_node.is_leaf() {
            side_nodes_iter.next();
            current_node = first_side_node.clone();

            // Advance the side node iterator to the next non-placeholder node. This may be either
            // another leaf node or an internal node.
            // If only placeholder nodes exist beyond the first leaf node, then that leaf node is,
            // in fact, the new root node.
            // Using `find(..)` advances the iterator beyond the next non-placeholder side node and
            // returns it. Therefore, we must consume the side node at this point. If another
            // non-placeholder node was found in the side node collection, merge it with the first
            // side node. This guarantees that the current node will be an internal node, and
            // not a leaf, by the time we start merging the remaining side nodes.
            if let Some(side_node) = side_nodes_iter.find(|side_node| !side_node.is_placeholder()) {
                current_node = Node::create_node_on_path(path, &current_node, side_node);
                self.storage
                    .insert(&current_node.hash(), current_node.as_buffer())?;
            }
        }

        for side_node in side_nodes_iter {
            current_node = Node::create_node_on_path(path, &current_node, side_node);
            self.storage
                .insert(&current_node.hash(), current_node.as_buffer())?;
        }

        self.set_root_node(current_node);

        Ok(())
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
        let tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();
        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..1 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..2 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_3() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..3 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_5() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..5 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_10() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..10 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "21ca4917e99da99a61de93deaf88c400d4c082991cb95779e444d43dd13e8849";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_100() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..100 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let root = tree.root();
        let expected_root = "82bf747d455a55e2f7044a03536fc43f1f55d43b855e72c0110c986707a23e4d";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_empty() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        let key = 0_u32.to_be_bytes();
        let data = [0_u8; 0];
        let _ = tree.update(&key, &data);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_delete_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        let key = 1_u32.to_be_bytes();
        let data = "DATA".as_bytes();
        let _ = tree.update(&key, data);
        let _ = tree.delete(&key);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_update_empty() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        let key = 0_u32.to_be_bytes();
        let data = "DATA".as_bytes();
        let _ = tree.update(&key, data);
        let _ = tree.update(&key, &[0; 0]);

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2_delete_1() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..2 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let key = 0_u32.to_be_bytes();
        let _ = tree.delete(&key);

        let root = tree.root();
        let expected_root = "d7cb6616832899ac111a852ca8df2d63a1cdb36cb84651ffde72e264506a456f";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_interleaved_update_delete() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        let data = b"DATA";

        for i in 0_u32..10 {
            let key = i.to_be_bytes();
            tree.update(&key, data).unwrap();
        }

        for i in 5_u32..15 {
            let key = i.to_be_bytes();
            tree.delete(&key).unwrap();
        }

        for i in 10_u32..20 {
            let key = i.to_be_bytes();
            tree.update(&key, data).unwrap();
        }

        for i in 15_u32..25 {
            let key = i.to_be_bytes();
            tree.delete(&key).unwrap();
        }

        for i in 20_u32..30 {
            let key = i.to_be_bytes();
            tree.update(&key, data).unwrap();
        }

        for i in 25_u32..35 {
            let key = i.to_be_bytes();
            tree.delete(&key).unwrap();
        }

        let root = tree.root();
        let expected_root = "7e6643325042cfe0fc76626c043b97062af51c7e9fc56665f12b479034bce326";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_sparse_update() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..50 {
            let key = (i * 2).to_be_bytes();
            tree.update(&key, b"DATA").unwrap();
        }

        let root = tree.root();
        let expected_root = "e02e761efef33aaa7a7027b4f5596c4c860476f299cdd0c4555199292d5041ee";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_sparse_update_delete() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        let data = b"DATA";

        for i in 0_u32..100 {
            let key = i.to_be_bytes();
            tree.update(&key, data).unwrap();
        }

        for i in 0_u32..50 {
            let key = (i * 2 + 1).to_be_bytes();
            tree.delete(&key).unwrap();
        }

        let root = tree.root();
        let expected_root = "e02e761efef33aaa7a7027b4f5596c4c860476f299cdd0c4555199292d5041ee";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_delete_non_existent_key_does_not_change_root() {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage).unwrap();

        for i in 0_u32..5 {
            let key = i.to_be_bytes();
            let data = "DATA".as_bytes();
            let _ = tree.update(&key, data);
        }

        let key = 1024_u32.to_be_bytes();
        let _ = tree.delete(&key);

        let root = tree.root();
        let expected_root = "108f731f2414e33ae57e584dc26bd276db07874436b2264ca6e520c658185c6b";
        assert_eq!(hex::encode(root), expected_root);
    }
}
