use crate::common::{AsPathIterator, Buffer, Bytes32};
use fuel_storage::Storage;

use crate::sparse::{zero_sum, Node, StorageNode};

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("Error")]
    Error(),
}

pub struct MerkleTree<'storage, StorageError> {
    root: Bytes32,
    storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>,
}

impl<'a, 'storage, StorageError> MerkleTree<'storage, StorageError>
where
    StorageError: std::error::Error + Clone,
{
    pub fn new(storage: &'storage mut dyn Storage<Bytes32, Buffer, Error = StorageError>) -> Self {
        Self {
            root: *zero_sum(),
            storage,
        }
    }

    pub fn update(&'a mut self, key: &Bytes32, data: &Bytes32) {
        self.root = self.update_for_root(key, data);
    }

    // PRIVATE

    fn update_for_root(&'a self, key: &Bytes32, _data: &Bytes32) -> Bytes32 {
        let col = self.path_set_for_root(key, &self.root);
        let root = self.update_with_path_set(col.as_slice());
        root
    }

    fn path_set_for_root(
        &'a self,
        leaf_key: &Bytes32,
        root: &Bytes32,
    ) -> Vec<(StorageNode<StorageError>, StorageNode<StorageError>)> {
        let leaf_buffer = self.storage.get(leaf_key).unwrap().unwrap();
        let leaf_node = Node::from_buffer(leaf_buffer.into_owned());
        let leaf_storage = StorageNode::<StorageError>::new(self.storage, leaf_node);

        let root_buffer = self.storage.get(root).unwrap().unwrap();
        let root_node = Node::from_buffer(root_buffer.into_owned());
        let root_storage = StorageNode::<StorageError>::new(self.storage, root_node);

        let iter = leaf_storage.as_path_iter(&root_storage);
        let mut path: Vec<(StorageNode<StorageError>, StorageNode<StorageError>)> = iter.collect();
        path.reverse();
        path
    }

    fn update_with_path_set(
        &'a self,
        path_set: &[(StorageNode<StorageError>, StorageNode<StorageError>)],
    ) -> Bytes32 {
        todo!()

        // 1. Set the leaf node in storage (i.e. path[0])
        // 1. Check the common prefix: the right-most bits that both the old path nad new path have in common
        // 2. For each pair of node and side node, hash them together to form the parent node, until
        //    we reach the new root
        // 3. Return the new root
    }

    fn delete_with_path_set(
        &'a self,
        path_set: &[(StorageNode<StorageError>, StorageNode<StorageError>)],
    ) -> Bytes32 {
        todo!()
    }
}

// 32:              (2^32)-1
//                    /  \
//                   /    \
//    (2^32)-(2^31)-1      (2^32)+(2^31)-1
//                 /       |
//               ...       0
//               /
//              /
//  2:        03
//           /  \
//          /    \
//  1:     01      05
//        /  \     |
//  0:   00  02    0
//       |   |
//       D   0

#[cfg(test)]
mod test {
    #[test]
    fn test_init() {}
}
