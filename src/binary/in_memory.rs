use crate::binary::Node;
use crate::common::Bytes32;
use crate::{binary, common};

type StorageMap = common::StorageMap<u64, Node>;
type BinaryMerkleTree = binary::MerkleTree<StorageMap>;

pub struct MerkleTree {
    tree: BinaryMerkleTree,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            tree: BinaryMerkleTree::new(StorageMap::new()),
        }
    }

    pub fn push(&mut self, data: &[u8]) {
        let _ = self.tree.push(data);
    }

    pub fn root(&mut self) -> Bytes32 {
        self.tree.root().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use binary::{empty_sum, leaf_sum, node_sum};
    use fuel_merkle_test_helpers::TEST_DATA;

    #[test]
    fn root_returns_the_empty_root_for_0_leaves() {
        let mut tree = MerkleTree::new();

        let root = tree.root();
        assert_eq!(root, empty_sum().clone());
    }

    #[test]
    fn root_returns_the_merkle_root_for_1_leaf() {
        let mut tree = MerkleTree::new();

        let data = &TEST_DATA[0..1]; // 1 leaf
        for datum in data.iter() {
            let _ = tree.push(datum);
        }

        let leaf_0 = leaf_sum(data[0]);

        let root = tree.root();
        assert_eq!(root, leaf_0);
    }

    #[test]
    fn root_returns_the_merkle_root_for_7_leaves() {
        let mut tree = MerkleTree::new();

        let data = &TEST_DATA[0..7]; // 7 leaves
        for datum in data.iter() {
            let _ = tree.push(datum);
        }

        let leaf_0 = leaf_sum(data[0]);
        let leaf_1 = leaf_sum(data[1]);
        let leaf_2 = leaf_sum(data[2]);
        let leaf_3 = leaf_sum(data[3]);
        let leaf_4 = leaf_sum(data[4]);
        let leaf_5 = leaf_sum(data[5]);
        let leaf_6 = leaf_sum(data[6]);

        let node_1 = node_sum(&leaf_0, &leaf_1);
        let node_5 = node_sum(&leaf_2, &leaf_3);
        let node_3 = node_sum(&node_1, &node_5);
        let node_9 = node_sum(&leaf_4, &leaf_5);
        let node_11 = node_sum(&node_9, &leaf_6);
        let node_7 = node_sum(&node_3, &node_11);

        let root = tree.root();
        assert_eq!(root, node_7);
    }
}
