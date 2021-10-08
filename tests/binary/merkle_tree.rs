use fuel_merkle::binary::MerkleTree;
use fuel_merkle::common::StorageMap;
use fuel_merkle_test_helpers::binary::MerkleTree as MockMerkleTree;

#[test]
fn test_prove() {
    let mut reference_merkle_tree = MockMerkleTree::new();
    reference_merkle_tree.set_proof_index(0);

    let mut storage = StorageMap::new();
    let mut merkle_tree = MerkleTree::new(&mut storage);

    let data = (0u32..10_000u32).into_iter();
    for datum in data {
        let _ = merkle_tree.push(&datum.to_be_bytes());
        let _ = reference_merkle_tree.push(&datum.to_be_bytes());
    }

    let reference_proof_set = reference_merkle_tree.prove().1;
    let proof_set = merkle_tree.prove(0).unwrap().1;
    assert_eq!(proof_set, reference_proof_set);
}
