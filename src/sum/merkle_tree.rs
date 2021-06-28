use crate::proof_set::ProofSet;
use crate::sum::data_pair::join_data_pair;
use crate::sum::hash::{empty_sum, leaf_sum, node_sum, Data};
use crate::sum::node::Node;

type DataNode = Node<Data>;

pub struct MerkleTree {
    head: Option<Box<DataNode>>,
    leaves_count: u64,
    proof_index: u64,
    proof_set: ProofSet,
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            head: None,
            leaves_count: 0,
            proof_index: 0,
            proof_set: ProofSet::new(),
        }
    }

    pub fn set_proof_index(&mut self, proof_index: u64) {
        if self.head().is_some() {
            panic!("Cannot change the proof index after adding a leaf!");
        }
        self.proof_index = proof_index;
    }

    pub fn root(&self) -> Data {
        match self.head() {
            None => empty_sum().clone(),
            Some(ref head) => {
                let mut current = head.clone();
                while current.next().is_some() {
                    let mut node = current;
                    let mut next_node = node.take_next().unwrap();
                    current = Self::join_subtrees(&mut next_node, &node)
                }
                current.data().clone()
            }
        }
    }

    pub const fn leaves_count(&self) -> u64 {
        self.leaves_count
    }

    pub fn push(&mut self, data: &[u8], fee: u64) {
        let node = Self::create_node(self.head.take(), 0, leaf_sum(data), fee);

        if self.leaves_count == self.proof_index {
            let sum_data = &join_data_pair(fee, &leaf_sum(data));
            self.proof_set.push(sum_data);
        }

        self.head = Some(node);
        self.join_all_subtrees();

        self.leaves_count += 1;
    }

    pub fn prove(mut self) -> (Data, ProofSet) {
        let proof_set_length = self.proof_set.len() as u32;

        if self.head().is_none() || proof_set_length == 0 {
            return (self.root(), self.proof_set);
        }

        let mut current = self.head().clone().unwrap();
        while current.next().is_some() && current.next_height().unwrap() < proof_set_length - 1 {
            let mut node = current;
            let mut next_node = node.take_next().unwrap();
            current = Self::join_subtrees(&mut next_node, &node)
        }

        if current.next().is_some() && current.next_height().unwrap() == proof_set_length - 1 {
            let fee = current.fee();
            let data = current.data();
            let sum_data = &join_data_pair(fee, data);
            self.proof_set.push(sum_data);
            current = current.take_next().unwrap();
        }

        while current.next().is_some() {
            let fee = current.next_fee().unwrap();
            let data = current.next_data().unwrap();
            let sum_data = &join_data_pair(fee, data);
            self.proof_set.push(sum_data);
            current = current.take_next().unwrap();
        }

        (self.root(), self.proof_set)
    }

    //
    // PRIVATE
    //

    fn head(&self) -> &Option<Box<DataNode>> {
        &self.head
    }

    fn join_all_subtrees(&mut self) {
        loop {
            let head = self.head.as_ref().unwrap();
            if !(head.next().is_some() && head.height() == head.next_height().unwrap()) {
                break;
            }

            let proof_set_length = self.proof_set.len() as u32;
            if head.height() + 1 == proof_set_length {
                let head_leaves_count = 1u64 << head.height();
                let mid = (self.leaves_count / head_leaves_count) * head_leaves_count;
                if self.proof_index < mid {
                    let fee = head.fee();
                    let data = head.data();
                    let sum_data = &join_data_pair(fee, data);
                    self.proof_set.push(sum_data);
                } else {
                    let fee = head.next_fee().unwrap();
                    let data = head.next_data().unwrap();
                    let sum_data = &join_data_pair(fee, data);
                    self.proof_set.push(sum_data);
                }
            }

            // Merge the two front nodes of the list into a single node
            let mut node = self.head.take().unwrap();
            let mut next_node = node.take_next().unwrap();
            let joined_node = Self::join_subtrees(&mut next_node, &node);

            self.head = Some(joined_node);
        }
    }

    fn join_subtrees(a: &mut DataNode, b: &DataNode) -> Box<DataNode> {
        let next = a.take_next();
        let height = a.height() + 1;
        let data = node_sum(a.fee(), a.data(), b.fee(), b.data());
        let fee = a.fee() + b.fee();
        Self::create_node(next, height, data, fee)
    }

    fn create_node(
        next: Option<Box<DataNode>>,
        height: u32,
        data: Data,
        fee: u64,
    ) -> Box<DataNode> {
        Box::new(DataNode::new(next, height, data, fee))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sum::data_pair::split_data_pair;
    use sha2::{Digest, Sha256 as Hash};
    use std::convert::TryFrom;

    type MT = MerkleTree;

    const NODE: u8 = 0x01;
    const LEAF: u8 = 0x00;

    fn empty_data() -> Data {
        let hash = Hash::new();
        <Data>::try_from(hash.finalize()).unwrap()
    }

    fn leaf_data(data: &[u8]) -> Data {
        let mut hash = Hash::new();
        hash.update(&[LEAF]);
        hash.update(&data);
        hash.finalize()
    }

    fn node_data(lhs_fee: u64, lhs_data: &[u8], rhs_fee: u64, rhs_data: &[u8]) -> Data {
        let mut hash = Hash::new();
        hash.update(&[NODE]);
        hash.update(&lhs_fee.to_be_bytes());
        hash.update(&lhs_data);
        hash.update(&rhs_fee.to_be_bytes());
        hash.update(&rhs_data);
        hash.finalize()
    }

    const FEE: u64 = 100;
    const DATA: [&[u8]; 10] = [
        "Frankly, my dear, I don't give a damn.".as_bytes(),
        "I'm going to make him an offer he can't refuse".as_bytes(),
        "Toto, I've got a feeling we're not in Kansas anymore.".as_bytes(),
        "Here's looking at you, kid.".as_bytes(),
        "Go ahead, make my day.".as_bytes(),
        "May the Force be with you.".as_bytes(),
        "You talking to me?".as_bytes(),
        "What we've got here is failure to communicate.".as_bytes(),
        "I love the smell of napalm in the morning.".as_bytes(),
        "Love means never having to say you're sorry.".as_bytes(),
    ];

    #[test]
    fn root_returns_the_hash_of_the_empty_string_when_no_leaves_are_pushed() {
        let mt = MT::new();
        let root = mt.root();

        let expected = empty_data();
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_leaf_when_one_leaf_is_pushed() {
        let mut mt = MT::new();

        let data = &DATA[0];
        mt.push(&data, FEE);
        let root = mt.root();

        let expected = leaf_data(&data);
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_4_leaves_are_pushed() {
        let mut mt = MT::new();

        let data = &DATA[0..4]; // 4 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }
        let root = mt.root();

        //       N3
        //      /  \
        //     /    \
        //   N1      N2
        //  /  \    /  \
        // L1  L2  L3  L4

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);

        let expected = node_3;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_5_leaves_are_pushed() {
        let mut mt = MT::new();

        let data = &DATA[0..5]; // 5 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }
        let root = mt.root();

        //          N4
        //         /  \
        //       N3    \
        //      /  \    \
        //     /    \    \
        //   N1      N2   \
        //  /  \    /  \   \
        // L1  L2  L3  L4  L5

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);
        let leaf_5 = leaf_data(&data[4]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_4 = node_data(FEE * 4, &node_3, FEE * 1, &leaf_5);

        let expected = node_4;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_7_leaves_are_pushed() {
        let mut mt = MT::new();

        let data = &DATA[0..7]; // 7 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }
        let root = mt.root();

        //              N6
        //          /        \
        //         /          \
        //       N4            N5
        //      /  \           /\
        //     /    \         /  \
        //   N1      N2      N3   \
        //  /  \    /  \    /  \   \
        // L1  L2  L3  L4  L5  L6  L7

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);
        let leaf_5 = leaf_data(&data[4]);
        let leaf_6 = leaf_data(&data[5]);
        let leaf_7 = leaf_data(&data[6]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 1, &leaf_5, FEE * 1, &leaf_6);
        let node_4 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_5 = node_data(FEE * 2, &node_3, FEE * 1, &leaf_7);
        let node_6 = node_data(FEE * 4, &node_4, FEE * 3, &node_5);

        let expected = node_6;
        assert_eq!(root, expected);
    }

    #[test]
    fn leaves_count_returns_the_number_of_leaves_pushed_to_the_tree() {
        let mut mt = MT::new();

        let data = &DATA[0..4];
        for datum in data.iter() {
            mt.push(datum, FEE);
        }

        assert_eq!(mt.leaves_count(), data.len() as u64);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index() {
        let mut mt = MT::new();
        mt.set_proof_index(0);

        let data = &DATA[0..4]; // 4 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        //       N3
        //      /  \
        //     /    \
        //   N1      N2
        //  /  \    /  \
        // L1  L2  L3  L4

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);

        let s_1 = set.get(0).unwrap();
        let (fee_1, data_1) = split_data_pair(s_1);
        let s_2 = set.get(1).unwrap();
        let (fee_2, data_2) = split_data_pair(s_2);
        let s_3 = set.get(2).unwrap();
        let (fee_3, data_3) = split_data_pair(s_3);

        assert_eq!(root, node_3);

        assert_eq!(fee_1, FEE);
        assert_eq!(data_1, &leaf_1[..]);

        assert_eq!(fee_2, FEE);
        assert_eq!(data_2, &leaf_2[..]);

        assert_eq!(fee_3, FEE * 2);
        assert_eq!(data_3, &node_2[..]);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_left_of_the_root() {
        let mut mt = MT::new();
        mt.set_proof_index(2);

        let data = &DATA[0..5]; // 5 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        //          N4
        //         /  \
        //       N3    \
        //      /  \    \
        //     /    \    \
        //   N1      N2   \
        //  /  \    /  \   \
        // L1  L2  L3  L4  L5

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);
        let leaf_5 = leaf_data(&data[4]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_4 = node_data(FEE * 4, &node_3, FEE * 1, &leaf_5);

        let s_1 = set.get(0).unwrap();
        let (fee_1, data_1) = split_data_pair(s_1);

        let s_2 = set.get(1).unwrap();
        let (fee_2, data_2) = split_data_pair(s_2);

        let s_3 = set.get(2).unwrap();
        let (fee_3, data_3) = split_data_pair(s_3);

        let s_4 = set.get(3).unwrap();
        let (fee_4, data_4) = split_data_pair(s_4);

        assert_eq!(root, node_4);

        assert_eq!(data_1, &leaf_3[..]);
        assert_eq!(fee_1, FEE);

        assert_eq!(data_2, &leaf_4[..]);
        assert_eq!(fee_2, FEE);

        assert_eq!(data_3, &node_1[..]);
        assert_eq!(fee_3, FEE * 2);

        assert_eq!(data_4, &leaf_5[..]);
        assert_eq!(fee_4, FEE);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_right_of_the_root() {
        let mut mt = MT::new();
        mt.set_proof_index(4);

        let data = &DATA[0..5]; // 5 leaves
        for datum in data.iter() {
            mt.push(datum, FEE);
        }

        let proof = mt.prove();
        let root = proof.0;
        let set = proof.1;

        //          N4
        //         /  \
        //       N3    \
        //      /  \    \
        //     /    \    \
        //   N1      N2   \
        //  /  \    /  \   \
        // L1  L2  L3  L4  L5

        let leaf_1 = leaf_data(&data[0]);
        let leaf_2 = leaf_data(&data[1]);
        let leaf_3 = leaf_data(&data[2]);
        let leaf_4 = leaf_data(&data[3]);
        let leaf_5 = leaf_data(&data[4]);

        let node_1 = node_data(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_data(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_data(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_4 = node_data(FEE * 4, &node_3, FEE * 1, &leaf_5);

        let s_1 = set.get(0).unwrap();
        let (fee_1, data_1) = split_data_pair(s_1);

        let s_2 = set.get(1).unwrap();
        let (fee_2, data_2) = split_data_pair(s_2);

        assert_eq!(root, node_4);

        assert_eq!(data_1, &leaf_5[..]);
        assert_eq!(fee_1, FEE);

        assert_eq!(data_2, &node_3[..]);
        assert_eq!(fee_2, FEE * 4);
    }

    #[test]
    fn prove_returns_the_root_of_the_empty_merkle_tree_when_no_leaves_are_added() {
        let mt = MT::new();

        let proof = mt.prove();
        let root = proof.0;

        let expected_root = empty_data();
        assert_eq!(root, expected_root);
    }

    #[test]
    fn prove_returns_an_empty_proof_set_when_no_leaves_are_added() {
        let mt = MT::new();

        let proof = mt.prove();
        let set = proof.1;

        assert_eq!(set.len(), 0);
    }
}