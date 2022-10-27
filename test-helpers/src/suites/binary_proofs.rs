use fuel_merkle::binary::MerkleTree;
use fuel_merkle::common::{Bytes32, StorageMap};

use fuel_merkle_test_helpers::data::{binary::ProofTest, EncodedValue, Encoding};

use digest::Digest;
use rand::seq::IteratorRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use sha2::Sha256;
use std::convert::TryInto;

type Hash = Sha256;

pub fn sum(data: &[u8]) -> Bytes32 {
    let mut hash = Hash::new();
    hash.update(&data);
    hash.finalize().try_into().unwrap()
}

fn generate_test(
    name: String,
    description: String,
    sample_data: &Vec<Bytes32>,
    proof_index: u64,
) -> ProofTest {
    let (root, proof_set) = {
        let storage = StorageMap::new();
        let mut test_tree = MerkleTree::new(storage);
        for datum in sample_data.iter() {
            test_tree.push(datum).unwrap();
        }
        // SAFETY: prove(i) is guaranteed to return a valid proof if the proof
        // index is within the range of valid leaves. proof_index will always
        // be selected from this range.
        test_tree.prove(proof_index).unwrap()
    };

    // SAFETY: All EncodedValues are specified with a valid encoding.
    let encoded_root = EncodedValue::from_raw(&root, Encoding::Hex);
    let encoded_proof_set = proof_set
        .iter()
        .map(|v| EncodedValue::from_raw(&v, Encoding::Hex))
        .collect::<Vec<_>>();
    let num_leaves = sample_data.len() as u64;

    ProofTest {
        name,
        description,
        root: encoded_root,
        proof_set: encoded_proof_set,
        proof_index,
        num_leaves,
        expected_verification: true,
    }
}

fn write_test(test: &ProofTest) {
    let yaml = serde_yaml::to_string(test).expect("Unable to serialize test!");
    std::fs::write(
        format!("../tests-data-binary/fixtures/{}.yaml", test.name),
        yaml,
    )
    .expect("Unable to write file!");
}

fn main() {
    let test_data_count = 2u64.pow(16);
    let test_data = (0..test_data_count)
        .map(|i| sum(&i.to_be_bytes()))
        .collect::<Vec<Bytes32>>();

    let mut rng = ChaCha8Rng::seed_from_u64(90210);

    let name = "Test 10 Leaves Index 4".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 10 leaves and leaf index 4. \
        This proof is valid and verification is expected to pass."
        .to_string();
    let samples = 10;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 4;
    let test = generate_test(name, description, &sample_data, proof_index);
    write_test(&test);

    let name = "Test 1 Leaf Index 0".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 1 leaf and leaf index 0. \
        This proof is valid and verification is expected to pass."
        .to_string();
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 0;
    let test = generate_test(name, description, &sample_data, proof_index);
    write_test(&test);

    let name = "Test 100 Leaves Index 10".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 100 leaves and leaf index 10. \
        This proof is valid and verification is expected to pass."
        .to_string();
    let samples = 100;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 10;
    let test = generate_test(name, description, &sample_data, proof_index);
    write_test(&test);

    let name = "Test 1024 Leaves Index 512".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 1024 leaves and leaf index 512. \
        This proof is valid and verification is expected to pass."
        .to_string();
    let samples = 1024;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 512;
    let test = generate_test(name, description, &sample_data, proof_index);
    write_test(&test);

    // 0 leaves; should fail
    let name = "Test 0 Leaves".to_string();
    let description = "\
        Build a proof from a binary Merkle tree and manual set the number of leaves to 0. \
        Setting the number of leaves to 0 implies that the source tree is empty. \
        This proof is invalid because empty trees cannot produce a proof. \
        Verification is expected to fail."
        .to_string();
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 0;
    let mut test = generate_test(name, description, &sample_data, proof_index);
    test.num_leaves = 0;
    test.expected_verification = false;
    write_test(&test);

    // Invalid proof index; should fail
    let name = "Test 1 Leaf Invalid Proof Index".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 1 leaf and manually set the leaf index to 1. \
        Because the leaf index is zero-based, leaf index 1 refers to a position outside the range of the source tree. \
        This proof is invalid because the leaf index is out of range. \
        Verification is expected to fail."
        .to_string();
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 0;
    let mut test = generate_test(name, description, &sample_data, proof_index);
    test.proof_index = 1;
    test.expected_verification = false;
    write_test(&test);

    // Invalid root; should fail
    let name = "Test 1 Leaf Invalid Root".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 1 leaf and manually set the root. \
        The root is manually set to the SHA256 hash of the string \"invalid\". \
        This proof is invalid because root is not generated from canonical Merkle tree construction. \
        Verification is expected to fail."
        .to_string();
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let proof_index = 0;
    let mut test = generate_test(name, description, &sample_data, proof_index);
    test.root = EncodedValue::new(hex::encode(sum(b"invalid")), Encoding::Hex);
    test.expected_verification = false;
    write_test(&test);

    // Invalid root; should fail
    let name = "Test 1024 Leaves Invalid Root".to_string();
    let description = "\
        Build a proof from a binary Merkle tree consisting of 1024 leaves and manually set the root. \
        The root is manually set to the SHA256 hash of the string \"invalid\". \
        This proof is invalid because root is not generated from canonical Merkle tree construction. \
        Verification is expected to fail."
        .to_string();
    let samples = 1024;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 512;
    let mut test = generate_test(name, description, &sample_data, index);
    test.root = EncodedValue::new(hex::encode(sum(b"invalid")), Encoding::Hex);
    test.expected_verification = false;
    write_test(&test);
}
