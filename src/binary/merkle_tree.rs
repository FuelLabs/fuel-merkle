use crate::binary::node::Node;
use crate::digest::Digest;
use bytes::Bytes;
use std::convert::TryFrom;
use std::marker::PhantomData;

const NODE: [u8; 1] = [0x01];
const LEAF: [u8; 1] = [0x00];

type Data = [u8; 32];
type DataRef<'a> = &'a [u8];
type DataNode = Node<Data>;

pub struct ProofSet {
    storage: Vec<Bytes>,
}

impl ProofSet {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn push(&mut self, data: &[u8]) {
        self.storage.push(Bytes::copy_from_slice(data))
    }

    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let d = self.storage.get(index);
        match d {
            None => None,
            Some(b) => Some(&b[..]),
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

#[cfg(test)]
mod proof_set_test {
    use super::*;

    #[test]
    fn test_it() {
        let mut set = ProofSet::new();

        let data = "Hello World";
        set.push(data.as_bytes());

        let d = set.get(0).expect("Can't get at index");
        println!("{:?}", d);
    }
}

pub struct MerkleTree<D: Digest> {
    head: Option<Box<DataNode>>,
    leaves_count: u64,
    proof_index: u64,
    proof_set: ProofSet,

    phantom: PhantomData<D>,
}

impl<D: Digest> MerkleTree<D> {
    pub fn new() -> Self {
        Self {
            head: None,
            leaves_count: 0,
            proof_index: 0,
            proof_set: ProofSet::new(),

            phantom: PhantomData,
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
            None => Self::empty_sum(),
            Some(ref head) => {
                let mut current = head.clone();
                while current.next().is_some() {
                    let mut node = current;
                    let mut next_node = node.take_next().unwrap();
                    current = Self::join_subtrees(&mut next_node, &node)
                }
                *current.data()
            }
        }
    }

    pub fn leaves_count(&self) -> u64 {
        self.leaves_count
    }

    pub fn push(&mut self, data: &[u8]) {
        if self.leaves_count == self.proof_index {
            self.proof_set.push(data);
        }

        let node = Self::create_node(self.head.take(), 0, Self::leaf_sum(data));
        self.head = Some(node);
        self.join_all_subtrees();

        self.leaves_count += 1;
    }

    pub fn prove(mut self) -> (Data, ProofSet) {
        let proof_set_length = self.proof_set.len() as u32;

        if self.head().is_none() || proof_set_length == 0 {
            return (self.root(), self.proof_set);
        }

        let mut current = self.head.clone().unwrap();
        while current.next().is_some() && current.next_height() + 1 < proof_set_length {
            let mut node = current;
            let mut next_node = node.take_next().unwrap();
            current = Self::join_subtrees(&mut next_node, &node)
        }

        if current.next().is_some() && current.next_height() + 1 == proof_set_length {
            self.proof_set.push(current.data());
            current = current.take_next().unwrap();
        }

        while current.next().is_some() {
            self.proof_set.push(current.next_data());
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
            if !(head.next().is_some() && head.height() == head.next_height()) {
                break;
            }

            let proof_set_length = self.proof_set.len() as u32;
            if head.height() + 1 == proof_set_length {
                let head_leaves_count = 1u64 << head.height();
                let mid = (self.leaves_count / head_leaves_count) * head_leaves_count;
                if self.proof_index < mid {
                    self.proof_set.push(head.data());
                } else {
                    self.proof_set.push(head.next_data());
                }
            }

            // Merge the two front nodes of the list into a single node
            let mut node = self.head.take().unwrap();
            let mut next_node = node.take_next().unwrap();
            let joined_node = Self::join_subtrees(&mut next_node, &node);

            self.head = Some(joined_node);
        }
    }

    // Merkle Tree hash of an empty list
    // MTH({}) = Hash()
    fn empty_sum() -> Data {
        let hash = D::new();
        let data = hash.finalize();

        <Data>::try_from(data.as_slice()).unwrap()
    }

    // Merkle tree hash of an n-element list D[n]
    // MTH(D[n]) = Hash(0x01 || MTH(D[0:k]) || MTH(D[k:n])
    fn node_sum(lhs_data: DataRef, rhs_data: DataRef) -> Data {
        let mut hash = D::new();

        hash.update(&NODE);
        hash.update(&lhs_data);
        hash.update(&rhs_data);
        let data = hash.finalize();

        <Data>::try_from(data.as_slice()).unwrap()
    }

    // Merkle tree hash of a list with one entry
    // MTH({d(0)}) = Hash(0x00 || d(0))
    fn leaf_sum(data: DataRef) -> Data {
        let mut hash = D::new();

        hash.update(&LEAF);
        hash.update(&data);
        let data = hash.finalize();

        <Data>::try_from(data.as_slice()).unwrap()
    }

    fn join_subtrees(a: &mut DataNode, b: &DataNode) -> Box<DataNode> {
        let next = a.take_next();
        let height = a.height() + 1;
        let data = Self::node_sum(a.data(), b.data());
        Self::create_node(next, height, data)
    }

    fn create_node(next: Option<Box<DataNode>>, height: u32, data: Data) -> Box<DataNode> {
        Box::new(DataNode::new(next, height, data))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sha::Sha256 as Hash;

    type MT = MerkleTree<Hash>;

    fn empty_data() -> Data {
        let hash = Hash::new();
        <Data>::try_from(hash.finalize()).unwrap()
    }

    fn leaf_data(data: DataRef) -> Data {
        let mut hash = Hash::new();
        hash.update(&LEAF);
        hash.update(&data);
        <Data>::try_from(hash.finalize()).unwrap()
    }
    fn node_data(lhs_data: DataRef, rhs_data: DataRef) -> Data {
        let mut hash = Hash::new();
        hash.update(&NODE);
        hash.update(&lhs_data);
        hash.update(&rhs_data);
        <Data>::try_from(hash.finalize()).unwrap()
    }

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

        let data = [1u8; 16];
        mt.push(&data);
        let root = mt.root();

        let expected = leaf_data(&data);
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_4_leaves_are_pushed() {
        let mut mt = MT::new();

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
        }
        let root = mt.root();

        //       N3
        //      /  \
        //     /    \
        //   N1      N2
        //  /  \    /  \
        // L1  L2  L3  L4

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&node_1, &node_2);

        let expected = node_3;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_5_leaves_are_pushed() {
        let mut mt = MT::new();

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
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

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);
        let leaf_5 = leaf_data(&leaves[4]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&node_1, &node_2);
        let node_4 = node_data(&node_3, &leaf_5);

        let expected = node_4;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_7_leaves_are_pushed() {
        let mut mt = MT::new();

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
            "Frankly, my dear, I don't give a damn.".as_bytes(),
            "Say hello to my little friend".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
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

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);
        let leaf_5 = leaf_data(&leaves[4]);
        let leaf_6 = leaf_data(&leaves[5]);
        let leaf_7 = leaf_data(&leaves[6]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&leaf_5, &leaf_6);
        let node_4 = node_data(&node_1, &node_2);
        let node_5 = node_data(&node_3, &leaf_7);
        let node_6 = node_data(&node_4, &node_5);

        let expected = node_6;
        assert_eq!(root, expected);
    }

    #[test]
    fn leaves_count_returns_the_number_of_leaves_pushed_to_the_tree() {
        let mut mt = MT::new();

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
        }

        assert_eq!(mt.leaves_count(), leaves.len() as u64);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index() {
        let mut mt = MT::new();
        mt.set_proof_index(0);

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
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

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);
        let leaf_5 = leaf_data(&leaves[4]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&node_1, &node_2);
        let node_4 = node_data(&node_3, &leaf_5);

        let s_1 = set.get(0).unwrap();
        let s_2 = set.get(1).unwrap();
        let s_3 = set.get(2).unwrap();
        let s_4 = set.get(3).unwrap();

        assert_eq!(root, node_4);
        assert_eq!(s_1, leaves[0]);
        assert_eq!(s_2, &leaf_2);
        assert_eq!(s_3, &node_2);
        assert_eq!(s_4, &leaf_5);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_on_the_left() {
        let mut mt = MT::new();
        mt.set_proof_index(2);

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
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

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);
        let leaf_5 = leaf_data(&leaves[4]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&node_1, &node_2);
        let node_4 = node_data(&node_3, &leaf_5);

        let s_1 = set.get(0).unwrap();
        let s_2 = set.get(1).unwrap();
        let s_3 = set.get(2).unwrap();
        let s_4 = set.get(3).unwrap();

        assert_eq!(root, node_4);
        assert_eq!(s_1, leaves[2]);
        assert_eq!(s_2, &leaf_4);
        assert_eq!(s_3, &node_1);
        assert_eq!(s_4, &leaf_5);
    }

    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_on_the_right() {
        let mut mt = MT::new();
        mt.set_proof_index(4);

        let leaves = [
            "Hello, World!".as_bytes(),
            "Making banana pancakes".as_bytes(),
            "What is love?".as_bytes(),
            "Bob Ross".as_bytes(),
            "The smell of napalm in the morning".as_bytes(),
        ];
        for leaf in leaves.iter() {
            mt.push(leaf);
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

        let leaf_1 = leaf_data(&leaves[0]);
        let leaf_2 = leaf_data(&leaves[1]);
        let leaf_3 = leaf_data(&leaves[2]);
        let leaf_4 = leaf_data(&leaves[3]);
        let leaf_5 = leaf_data(&leaves[4]);

        let node_1 = node_data(&leaf_1, &leaf_2);
        let node_2 = node_data(&leaf_3, &leaf_4);
        let node_3 = node_data(&node_1, &node_2);
        let node_4 = node_data(&node_3, &leaf_5);

        let s_1 = set.get(0).unwrap();
        let s_2 = set.get(1).unwrap();

        assert_eq!(root, node_4);
        assert_eq!(s_1, leaves[4]);
        assert_eq!(s_2, &node_3);
    }
}
