use fuel_merkle::binary::MerkleTree;
use fuel_merkle::common::{empty_sum_sha256, Bytes32, StorageMap};

use fuel_merkle_test_helpers::data::{
    binary::ProofTest, EncodedValue, ENCODING_BASE_64, ENCODING_HEX,
};

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

fn generate_test(name: String, sample_data: &Vec<Bytes32>, proof_index: u64) -> ProofTest {
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
    let encoded_root = EncodedValue::from_raw(&root, ENCODING_HEX).unwrap();
    let encoded_proof_set = proof_set
        .iter()
        .map(|v| EncodedValue::from_raw(&v, ENCODING_HEX).unwrap())
        .collect::<Vec<_>>();
    let proof_data =
        EncodedValue::from_raw(&sample_data[proof_index as usize], ENCODING_BASE_64).unwrap();
    let num_leaves = sample_data.len() as u64;

    ProofTest {
        name,
        root: encoded_root,
        proof_set: encoded_proof_set,
        proof_data,
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

    let samples = 10;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 4;
    let test = generate_test("10_leaves_index_4".to_string(), &sample_data, index);
    write_test(&test);

    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 0;
    let test = generate_test("1_leaf_index_0".to_string(), &sample_data, index);
    write_test(&test);

    let samples = 100;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 10;
    let test = generate_test("100_leaves_index_10".to_string(), &sample_data, index);
    write_test(&test);

    let samples = 1024;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 512;
    let test = generate_test("1024_leaves_index_512".to_string(), &sample_data, index);
    write_test(&test);

    // 0 leaves; should fail
    let test = ProofTest {
        name: "0_leaves".to_string(),
        root: EncodedValue::new(hex::encode(empty_sum_sha256()), ENCODING_HEX),
        proof_set: vec![],
        proof_data: EncodedValue::new("".to_string(), ENCODING_BASE_64),
        proof_index: 0,
        num_leaves: 0,
        expected_verification: false,
    };
    write_test(&test);

    // Invalid proof index; should fail
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 0;
    let mut test = generate_test("1_leaf_index_1".to_string(), &sample_data, index);
    test.proof_index = 1;
    test.expected_verification = false;
    write_test(&test);

    // Invalid root; should fail
    let samples = 1;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 0;
    let mut test = generate_test("1_leaf_invalid_root".to_string(), &sample_data, index);
    test.root = EncodedValue::new(hex::encode(sum(b"invalid")), ENCODING_HEX);
    test.expected_verification = false;
    write_test(&test);

    // Invalid root; should fail
    let samples = 1024;
    let sample_data = test_data.iter().cloned().choose_multiple(&mut rng, samples);
    let index = 512;
    let mut test = generate_test("1024_leaves_invalid_root".to_string(), &sample_data, index);
    test.root = EncodedValue::new(hex::encode(sum(b"invalid")), ENCODING_HEX);
    test.expected_verification = false;
    write_test(&test);
}
