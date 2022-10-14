use fuel_merkle::binary::MerkleTree;
use fuel_merkle::common::{Bytes32, StorageMap};

use fuel_merkle_test_helpers::data::{binary::ProofTest, EncodedValue, ENCODING_HEX};

use digest::Digest;
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use sha2::Sha256;
use std::convert::TryInto;

type Hash = Sha256;

pub fn sum(data: &[u8]) -> Bytes32 {
    let mut hash = Hash::new();
    hash.update(&data);
    hash.finalize().try_into().unwrap()
}

fn main() {
    let test_data_count = 2u64.pow(16);
    let test_data = (0..test_data_count)
        .map(|i| sum(&i.to_be_bytes()))
        .collect::<Vec<Bytes32>>();

    let mut rng = thread_rng();
    let samples = 10;

    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);

    let index = rng.gen_range(0..samples) as u64;
    let (root, proof_set) = {
        let storage = StorageMap::new();
        let mut test_tree = MerkleTree::new(storage);
        for datum in sample_data.iter() {
            test_tree.push(datum).unwrap();
        }
        test_tree.prove(index).unwrap()
    };

    let encoded_root = EncodedValue::new(hex::encode(root), ENCODING_HEX.to_string());
    let encoded_proof_set = proof_set
        .iter()
        .map(|v| EncodedValue::new(hex::encode(v), ENCODING_HEX.to_string()))
        .collect::<Vec<_>>();

    let test = ProofTest {
        name: "test test test".to_string(),
        root: encoded_root,
        proof_set: encoded_proof_set,
        proof_index: index,
        num_leaves: samples as u64,
        expected_verification: true,
    };

    let yaml = serde_yaml::to_string(&test).expect("Unable to serialize test!");
    std::fs::write(
        format!("../tests-data-binary/fixtures/{}.yaml", test.name),
        yaml,
    )
    .expect("Unable to write file!");
}
