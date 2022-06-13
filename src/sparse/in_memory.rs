use crate::common::Bytes32;
use crate::sparse::Buffer;
use crate::{common, sparse};

use std::pin::Pin;
use std::ptr::NonNull;

type StorageMap = common::StorageMap<Bytes32, Buffer>;
type SparseMerkleTree<'a> = sparse::MerkleTree<'a, StorageMap>;

pub struct MerkleTree<'a> {
    storage: StorageMap,
    tree: *mut SparseMerkleTree<'a>,
}

impl<'a> MerkleTree<'a> {
    pub fn new() -> Pin<Box<Self>> {
        let res = Self {
            storage: StorageMap::new(),
            tree: std::ptr::null_mut(),
        };

        let mut boxed = Box::pin(res);

        unsafe {
            let mut storage = NonNull::from(&boxed.storage);
            let mut tree = Box::pin(SparseMerkleTree::new(storage.as_mut()));
            boxed.tree = tree.as_mut().get_unchecked_mut();
        }

        boxed
    }

    pub fn update(&mut self, key: &Bytes32, data: &[u8]) {
        unsafe {
            self.tree
                .as_mut()
                .unwrap_unchecked()
                .update(key, data)
                .unwrap_unchecked();
        }
    }

    pub fn root(&self) -> Bytes32 {
        unsafe { self.tree.as_ref().unwrap_unchecked().root() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sparse::hash::sum;

    #[test]
    fn test_empty_root() {
        let tree = MerkleTree::new();
        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1() {
        let mut tree = MerkleTree::new();

        tree.update(&sum(b"\x00\x00\x00\x00"), b"DATA");

        let root = tree.root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
        assert_eq!(hex::encode(root), expected_root);
    }
}
