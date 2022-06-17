use crate::common::Bytes32;
use crate::sparse::Buffer;
use crate::{common, sparse};

use alloc::boxed::Box;
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::ptr::NonNull;

type StorageMap = common::StorageMap<Bytes32, Buffer>;
type SparseMerkleTree<'a> = sparse::MerkleTree<'a, StorageMap>;

pub struct MerkleTree<'a> {
    storage: StorageMap,
    tree: Option<SparseMerkleTree<'a>>,
    _marker: PhantomPinned,
}

impl<'a> MerkleTree<'a> {
    pub fn new() -> Pin<Box<Self>> {
        let res = Self {
            storage: StorageMap::new(),
            tree: None,
            _marker: PhantomPinned,
        };

        let mut boxed = Box::pin(res);

        unsafe {
            let mut storage = NonNull::from(&boxed.storage);
            boxed.as_mut().get_unchecked_mut().tree = Some(SparseMerkleTree::new(storage.as_mut()));
        }

        boxed
    }

    pub fn update(self: Pin<&mut Self>, key: &Bytes32, data: &[u8]) {
        unsafe {
            self.get_unchecked_mut()
                .tree
                .as_mut()
                .unwrap_unchecked()
                .update(key, data)
                .unwrap_unchecked();
        }
    }

    pub fn delete(self: Pin<&mut Self>, key: &Bytes32) {
        unsafe {
            self.get_unchecked_mut()
                .tree
                .as_mut()
                .unwrap_unchecked()
                .delete(key)
                .unwrap_unchecked();
        }
    }

    pub fn root(self: Pin<&Self>) -> Bytes32 {
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
        let root = tree.as_ref().root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1() {
        let mut tree = MerkleTree::new();

        tree.as_mut().update(&sum(b"\x00\x00\x00\x00"), b"DATA");

        let root = tree.as_ref().root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2() {
        let mut tree = MerkleTree::new();

        tree.as_mut().update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.as_mut().update(&sum(b"\x00\x00\x00\x01"), b"DATA");

        let root = tree.as_ref().root();
        let expected_root = "8d0ae412ca9ca0afcb3217af8bcd5a673e798bd6fd1dfacad17711e883f494cb";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_3() {
        let mut tree = MerkleTree::new();

        tree.as_mut().update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.as_mut().update(&sum(b"\x00\x00\x00\x01"), b"DATA");
        tree.as_mut().update(&sum(b"\x00\x00\x00\x02"), b"DATA");

        let root = tree.as_ref().root();
        let expected_root = "52295e42d8de2505fdc0cc825ff9fead419cbcf540d8b30c7c4b9c9b94c268b7";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_1_delete_1() {
        let mut tree = MerkleTree::new();

        tree.as_mut().update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.as_mut().delete(&sum(b"\x00\x00\x00\x00"));

        let root = tree.as_ref().root();
        let expected_root = "0000000000000000000000000000000000000000000000000000000000000000";
        assert_eq!(hex::encode(root), expected_root);
    }

    #[test]
    fn test_update_2_delete_1() {
        let mut tree = MerkleTree::new();

        tree.as_mut().update(&sum(b"\x00\x00\x00\x00"), b"DATA");
        tree.as_mut().update(&sum(b"\x00\x00\x00\x01"), b"DATA");
        tree.as_mut().delete(&sum(b"\x00\x00\x00\x01"));

        let root = tree.as_ref().root();
        let expected_root = "39f36a7cb4dfb1b46f03d044265df6a491dffc1034121bc1071a34ddce9bb14b";
        assert_eq!(hex::encode(root), expected_root);
    }
}
