use crate::common::Bytes32;
use crate::sparse::Buffer;
use crate::{common, sparse};

use std::pin::Pin;
use std::ptr::NonNull;

type StorageMap = common::StorageMap<Bytes32, Buffer>;
type SparseMerkleTree<'a> = sparse::MerkleTree<'a, StorageMap>;

pub struct MerkleTree<'a> {
    storage: StorageMap,
    tree_ptr: *mut SparseMerkleTree<'a>,
}

impl<'a> MerkleTree<'a> {
    pub fn new() -> Pin<Box<Self>> {
        let res = Self {
            storage: StorageMap::new(),
            tree_ptr: std::ptr::null_mut(),
        };

        let mut boxed = Box::pin(res);

        unsafe {
            let mut storage = NonNull::from(&boxed.storage);
            let mut tree = Box::pin(SparseMerkleTree::new(storage.as_mut()));
            boxed.tree_ptr = tree.as_mut().get_unchecked_mut();
        }

        boxed
    }

    pub fn update(&mut self, key: &Bytes32, data: &[u8]) {
        unsafe {
            self.tree_ptr
                .as_mut()
                .unwrap_unchecked()
                .update(key, data)
                .unwrap_unchecked();
        }
    }

    pub fn delete(&mut self, key: &Bytes32) {
        unsafe {
            self.tree_ptr
                .as_mut()
                .unwrap_unchecked()
                .delete(key)
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

    #[test]
    fn test_update_2() {
        let mut tree = MerkleTree::new();

        tree.update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.update(&sum(b"\x00\x00\x00\x01"), b"DATA");

        let root = tree.root();
        let expected_root = "8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_3() {
        let mut tree = MerkleTree::new();

        tree.update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.update(&sum(b"\x00\x00\x00\x01"), b"DATA");
        tree.update(&sum(b"\x00\x00\x00\x02"), b"DATA");

        let root = tree.root();
        let expected_root = "52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_delete_1() {
        let mut tree = MerkleTree::new();

        tree.update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.delete(&sum(b"\x00\x00\x00\x00"));

        let root = tree.root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2_delete_1() {
        let mut tree = MerkleTree::new();

        tree.update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.update(&sum(b"\x00\x00\x00\x01"), b"DATA");
        tree.delete(&sum(b"\x00\x00\x00\x01"));

        let root = tree.root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
        assert_eq!(hex::encode(root), expected_root);
    }
}
