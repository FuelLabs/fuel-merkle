use fuel_storage::Storage;

use crate::common::Position;
use crate::sparse::{empty_sum, leaf_sum, node_sum, zero_sum, Data, Node, Subtree};

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("Error")]
    Error(),
}

// type DataNode<'storage> = Node<'storage, Data, MerkleTreeError>;
type ProofSet = Vec<Data>;

pub struct MerkleTree<'storage, StorageError> {
    storage: &'storage mut dyn Storage<Data, Data, Error = StorageError>,
}

impl<'storage, StorageError> MerkleTree<'storage, StorageError>
where
    StorageError: std::error::Error + 'static,
{
    pub fn new(storage: &'storage mut dyn Storage<Data, Data, Error = StorageError>,
    ) -> Self {
        Self {
            storage,
        }
    }

    pub fn update() {}
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
    fn test_init() {

    }
}
