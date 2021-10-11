use fuel_merkle::binary::MerkleTree;
use fuel_merkle::common::StorageMap;
use fuel_merkle_test_helpers::binary::MerkleTree as MockMerkleTree;

const MAX_LEAVES: u64 = 1 << 8;
const TEST_DATA: std::ops::RangeInclusive<u64> = 0..=MAX_LEAVES;

#[test]
fn test_root() {
    let mut storage = StorageMap::new();
    let mut merkle_tree = MerkleTree::new(&mut storage);
    for datum in TEST_DATA {
        let _ = merkle_tree.push(&datum.to_be_bytes());
    }

    for _ in 0..MAX_LEAVES {
        let mut reference_merkle_tree = MockMerkleTree::new();
        for datum in TEST_DATA {
            let _ = reference_merkle_tree.push(&datum.to_be_bytes());
        }

        let root = merkle_tree.root().unwrap();
        let reference_root = reference_merkle_tree.root();
        assert_eq!(root, reference_root);
    }
}

#[test]
fn test_prove() {
    let mut storage = StorageMap::new();
    let mut merkle_tree = MerkleTree::new(&mut storage);
    for datum in TEST_DATA {
        let _ = merkle_tree.push(&datum.to_be_bytes());
    }

    for index in 0..MAX_LEAVES {
        let mut reference_merkle_tree = MockMerkleTree::new();
        reference_merkle_tree.set_proof_index(index);
        for datum in TEST_DATA {
            let _ = reference_merkle_tree.push(&datum.to_be_bytes());
        }

        let proof_set = merkle_tree.prove(index).unwrap().1;
        let reference_proof_set = reference_merkle_tree.prove().1;
        assert_eq!(proof_set, reference_proof_set);
    }
}
