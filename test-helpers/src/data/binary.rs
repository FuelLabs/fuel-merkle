use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use fuel_merkle::{
    binary::in_memory,
    common::{Bytes32, ProofSet},
};

use crate::{
    binary::verify,
    data::{EncodedValue, TestError},
};

const ACTION_PUSH: &str = "push";

trait MerkleTreeTestAdaptor {
    fn push(&mut self, data: &[u8]);
    fn proof(&mut self, proof_index: u64) -> Option<(Bytes32, ProofSet)>;
    fn root(&mut self) -> Bytes32;
}

#[derive(Serialize, Deserialize)]
struct Step {
    action: String,
    data: Option<EncodedValue>,
}

enum Action {
    Push(EncodedValue),
}

impl Step {
    pub fn execute(self, tree: &mut dyn MerkleTreeTestAdaptor) -> Result<(), TestError> {
        match self.action_type()? {
            Action::Push(encoded_data) => {
                let data_bytes = encoded_data.into_bytes()?;
                let data: &[u8] = &data_bytes;
                tree.push(data);
                Ok(())
            }
        }
    }

    // Translate the action string found in the step definition to an Action enum variant with the
    // appropriate key and data bindings.
    fn action_type(&self) -> Result<Action, TestError> {
        match self.action.as_str() {
            // An Update has a key and data
            ACTION_PUSH => Ok(Action::Push(self.data.clone().unwrap())),

            // Unsupported action
            _ => Err(TestError::UnsupportedAction(self.action.clone())),
        }
    }
}

struct InMemoryMerkleTreeTestAdaptor {
    tree: Box<in_memory::MerkleTree>,
}

impl MerkleTreeTestAdaptor for InMemoryMerkleTreeTestAdaptor {
    fn push(&mut self, data: &[u8]) {
        self.tree.as_mut().push(data)
    }

    fn root(&mut self) -> Bytes32 {
        self.tree.as_mut().root()
    }

    fn proof(&mut self, proof_index: u64) -> Option<(Bytes32, ProofSet)> {
        self.tree.prove(proof_index)
    }
}

#[derive(Serialize, Deserialize)]
struct RootTest {
    name: String,
    expected_root: EncodedValue,
    steps: Vec<Step>,
}

impl RootTest {
    pub fn execute(self) -> Result<(), TestError> {
        let tree = Box::new(in_memory::MerkleTree::new());
        let mut tree = InMemoryMerkleTreeTestAdaptor { tree };

        for step in self.steps {
            step.execute(&mut tree)?
        }

        let root = tree.root();
        let expected_root: Bytes32 = self.expected_root.into_bytes()?.try_into().unwrap();

        assert_eq!(root, expected_root);

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProofTest {
    pub name: String,
    pub root: EncodedValue,
    pub proof_set: Vec<EncodedValue>,
    pub proof_data: EncodedValue,
    pub proof_index: u64,
    pub num_leaves: u64,
    pub expected_verification: bool,
}

impl ProofTest {
    pub fn execute(self) -> Result<(), TestError> {
        let root: Bytes32 = self.root.into_bytes()?.as_slice().try_into().unwrap();
        let proof_set = self
            .proof_set
            .iter()
            .cloned()
            .map(|v| v.into_bytes().unwrap().as_slice().try_into().unwrap())
            .collect::<Vec<Bytes32>>();

        let verification = verify(&root, &proof_set, self.proof_index, self.num_leaves);

        if verification == self.expected_verification {
            Ok(())
        } else {
            Err(TestError::Failed(self.name))
        }
    }
}
