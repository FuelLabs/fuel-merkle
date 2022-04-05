#![feature(custom_test_frameworks)]
#![test_runner(datatest::runner)]

extern crate core;

use fuel_merkle::common::{Bytes32, StorageError, StorageMap};
use fuel_merkle::sparse::MerkleTree;
use serde::Deserialize;
use std::convert::TryInto;

const BUFFER_SIZE: usize = 69;
pub type Buffer = [u8; BUFFER_SIZE];

#[derive(Deserialize)]
struct EncodedValue {
    value: String,
    encoding: String,
}

impl EncodedValue {
    fn to_bytes(self) -> Vec<u8> {
        match self.encoding.as_str() {
            "hex" => hex::decode(self.value).unwrap(),
            "utf-8" => self.value.into_bytes(),

            // Unsupported encoding
            _ => panic!("unsupported!"),
        }
    }
}

#[derive(Deserialize)]
struct Step {
    action: String,
    key: Option<EncodedValue>,
    data: Option<EncodedValue>,
}

impl Step {
    pub fn execute(self, tree: &mut MerkleTree<StorageError>) {
        match self.action.as_str() {
            "update" => {
                let key_bytes = self.key.unwrap().to_bytes();
                let key = &key_bytes.try_into().unwrap();
                let data_bytes = self.data.unwrap().to_bytes();
                let data: &[u8] = &data_bytes;
                tree.update(key, data).unwrap();
            }
            "delete" => {
                let key_bytes = self.key.unwrap().to_bytes();
                let key = &key_bytes.try_into().unwrap();
                tree.delete(key).unwrap();
            }

            // Unsupported action
            _ => panic!("unsupported action!"),
        }
    }
}

#[derive(Deserialize)]
struct Test {
    name: String,
    expected_root: EncodedValue,
    steps: Vec<Step>,
}

impl Test {
    pub fn execute(self) {
        let mut storage = StorageMap::<Bytes32, Buffer>::new();
        let mut tree = MerkleTree::<StorageError>::new(&mut storage);

        for step in self.steps {
            step.execute(&mut tree)
        }

        let root = tree.root();
        let expected_root: Bytes32 = {
            let root_bytes = self.expected_root.to_bytes();
            root_bytes.try_into().unwrap()
        };

        assert_eq!(root, expected_root);
    }
}

#[datatest::data("tests/smt_test_spec.yaml")]
#[test]
fn test_data(test: Test) {
    println!("{}", test.name);
    test.execute()
}
