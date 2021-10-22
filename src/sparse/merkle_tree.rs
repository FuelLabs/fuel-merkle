use crate::common::Bytes32;
use fuel_storage::Storage;

use crate::sparse::{empty_sum, leaf_sum, node_sum, zero_sum, Node, Subtree};

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("Error")]
    Error(),
}

pub struct MerkleTree<'storage, StorageError> {
    storage: &'storage mut dyn Storage<Bytes32, Bytes32, Error = StorageError>,
}

impl<'storage, StorageError> MerkleTree<'storage, StorageError>
where
    StorageError: std::error::Error + 'static,
{
    pub fn new(storage: &'storage mut dyn Storage<Bytes32, Bytes32, Error = StorageError>) -> Self {
        Self { storage }
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
    #[test]
    fn test_init() {}
}
