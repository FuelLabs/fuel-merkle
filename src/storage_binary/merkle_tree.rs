use crate::common::{position::Position};

use crate::storage_binary::hash::{empty_sum, leaf_sum, node_sum, Data};
use crate::storage_binary::node::Node;
use crate::storage_binary::storage::Storage;

use crate::proof_set::ProofSet;

type DataNode = Node<Data>;

#[derive(Debug, Clone)]
pub struct Head<T> {
    node: T,
    next: Option<Box<Head<T>>>,
}

impl<T> Head<T> {
    pub fn new(node: T, next: Option<Box<Head<T>>>) -> Self {
        Self {
            node,
            next,
        }
    }

    pub fn next(&self) -> &Option<Box<Head<T>>> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<Head<T>>> {
        &mut self.next
    }

    pub fn take_next(&mut self) -> Option<Box<Head<T>>> {
        self.next_mut().take()
    }
}

pub struct MerkleTree<'storage> {
    storage: &'storage mut dyn Storage<Data, DataNode>,
    head: Option<Box<Head<DataNode>>>,
    leaves_count: u64,
}

impl<'storage> MerkleTree<'storage> {
    pub fn new(storage: &'storage mut dyn Storage<Data, DataNode>) -> Self {
        let mut tree = Self {
            storage,
            head: None,
            leaves_count: 0,
        };
        tree
    }

    pub fn root(&self) -> Data {
        empty_sum().clone()
    }

    pub fn prove(&mut self, proof_index: u64) -> (Data, ProofSet) {
        let mut proof_set = ProofSet::new();

        (self.root(), proof_set)
    }

    pub fn push(&mut self, data: &[u8]) {
        let position = Position::from_leaf_index(self.leaves_count);
        let leaf_sum = leaf_sum(data);
        let node = DataNode::new(position, leaf_sum.clone());
        self.storage.create(leaf_sum.clone(), node.clone());

        let next = self.head.take();
        let head = Box::new(Head::<DataNode>::new(node, next));
        println!("Setting head: {:?}", head);
        self.head = Some(head);
        self.join_all_subtrees();

        self.leaves_count += 1;
    }

    //
    // PRIVATE
    //

    fn join_all_subtrees(&mut self) {
        loop {
            let head = self.head.as_ref().unwrap();

            // if head.next.is_some() {
            //     println!("comparing {:?} and {:?}", head.node.position(), head.next().as_ref().map(|next| next.node.position()).unwrap());
            //     if head.node.position().height() == head.next().as_ref().map(|next| next.node.position().height()).unwrap() {
            //         println!("{:?} == {:?}", head.node.position().height(), head.next().as_ref().map(|next| next.node.position().height()).unwrap());
            //     } else {
            //         println!("{:?} != {:?}", head.node.position().height(), head.next().as_ref().map(|next| next.node.position().height()).unwrap());
            //     }
            // }

            if !(
                head.next().is_some() &&
                head.node.position().height() == head.next().as_ref().map(|next| next.node.position().height()).unwrap()
            ) {
                break;
            }

            // Merge the two front heads of the list into a single head
            let mut head_a = self.head.take().unwrap();
            let mut head_b = head_a.take_next().unwrap();
            let joined_head = self.join_subtrees(&mut head_b, &mut head_a);
            self.storage.create(joined_head.node.key(), joined_head.node.clone());

            println!("Setting head: {:?}", joined_head);
            self.head = Some(joined_head);
        }
        println!();
    }

    fn join_subtrees(&mut self, a: &mut Head<DataNode>, b: &mut Head<DataNode>) -> Box<Head<DataNode>> {
        print!("joining {:?}, {:?}... ", a.node.position(), b.node.position());

        let position = a.node.position().parent();
        let node_sum = node_sum(&a.node.key(), &b.node.key());
        let mut joined_node = DataNode::new(position, node_sum.clone());
        joined_node.set_left_key(Some(a.node.key()));
        joined_node.set_right_key(Some(b.node.key()));
        self.storage.create(node_sum, joined_node.clone());

        a.node.set_parent_key(Some(joined_node.key()));
        b.node.set_parent_key(Some(joined_node.key()));
        self.storage.update(a.node.key(), a.node.clone()).expect("Unable to update!");
        self.storage.update(b.node.key(), b.node.clone()).expect("Unable to update!");

        let joined_head = Head::new(joined_node, a.take_next());

        println!("created {:?}", joined_head.node.position());
        Box::new(joined_head)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage_binary::hash::Hash;
    use crate::storage_binary::storage_map::StorageMap;
    use digest::Digest;

    const NODE: u8 = 0x01;
    const LEAF: u8 = 0x00;

    fn empty_data() -> Data {
        let hash = Hash::new();
        hash.finalize()
    }
    fn leaf_data(data: &[u8]) -> Data {
        let mut hash = Hash::new();
        hash.update(&[LEAF]);
        hash.update(&data);
        hash.finalize()
    }
    fn node_data(lhs_data: &[u8], rhs_data: &[u8]) -> Data {
        let mut hash = Hash::new();
        hash.update(&[NODE]);
        hash.update(&lhs_data);
        hash.update(&rhs_data);
        hash.finalize()
    }

    const DATA: [&[u8]; 20] = [
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
    fn test_it() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MerkleTree::new(&mut storage_map);

        let data = &DATA[0..7]; // 7 leaves
        for datum in data.iter() {
            tree.push(datum);
        }

        //               07
        //              /  \
        //             /    \
        //            /      \
        //           /        \
        //          /          \
        //         /            \
        //       03              11
        //      /  \            /  \
        //     /    \          /    \
        //   01      05       09     \
        //  /  \    /  \     /  \     \
        // 00  02  04  06   08  10    12
        // 00  01  02  03   04  05    06

        let leaf_0 = leaf_data(&data[0]);
        let leaf_1 = leaf_data(&data[1]);
        let leaf_2 = leaf_data(&data[2]);
        let leaf_3 = leaf_data(&data[3]);
        let leaf_4 = leaf_data(&data[4]);
        let leaf_5 = leaf_data(&data[5]);
        let leaf_6 = leaf_data(&data[6]);

        let node_1 = node_data(&leaf_0, &leaf_1);
        let node_5 = node_data(&leaf_2, &leaf_3);
        let node_3 = node_data(&node_1, &node_5);
        let node_9 = node_data(&leaf_4, &leaf_5);
        let node_11 = node_data(&node_9, &leaf_6);
        let node_7 = node_data(&node_3, &node_11);

        let s_leaf_0 = storage_map.get(leaf_0).unwrap();
        assert_eq!(s_leaf_0.left_key(), None);
        assert_eq!(s_leaf_0.right_key(), None);
        assert_eq!(s_leaf_0.parent_key(), Some(node_1.clone()));

        let s_leaf_1 = storage_map.get(leaf_1).unwrap();
        assert_eq!(s_leaf_1.left_key(), None);
        assert_eq!(s_leaf_1.right_key(), None);
        assert_eq!(s_leaf_1.parent_key(), Some(node_1.clone()));

        let s_leaf_2 = storage_map.get(leaf_2).unwrap();
        assert_eq!(s_leaf_2.left_key(), None);
        assert_eq!(s_leaf_2.right_key(), None);
        assert_eq!(s_leaf_2.parent_key(), Some(node_5.clone()));

        let s_leaf_3 = storage_map.get(leaf_3).unwrap();
        assert_eq!(s_leaf_3.left_key(), None);
        assert_eq!(s_leaf_3.right_key(), None);
        assert_eq!(s_leaf_3.parent_key(), Some(node_5.clone()));

        let s_leaf_4 = storage_map.get(leaf_4.clone()).unwrap();
        assert_eq!(s_leaf_4.left_key(), None);
        assert_eq!(s_leaf_4.right_key(), None);
        assert_eq!(s_leaf_4.parent_key(), Some(node_9.clone()));

        let s_leaf_5 = storage_map.get(leaf_5).unwrap();
        assert_eq!(s_leaf_5.left_key(), None);
        assert_eq!(s_leaf_5.right_key(), None);
        assert_eq!(s_leaf_5.parent_key(), Some(node_9.clone()));

        let s_leaf_6 = storage_map.get(leaf_6).unwrap();
        assert_eq!(s_leaf_6.left_key(), None);
        assert_eq!(s_leaf_6.right_key(), None);
        assert_eq!(s_leaf_6.parent_key(), None);

        let s_node_1 = storage_map.get(node_1).unwrap();
        assert_eq!(s_node_1.left_key(), Some(leaf_0.clone()));
        assert_eq!(s_node_1.right_key(), Some(leaf_1.clone()));
        assert_eq!(s_node_1.parent_key(), Some(node_3.clone()));

        let s_node_5 = storage_map.get(node_5).unwrap();
        assert_eq!(s_node_5.left_key(), Some(leaf_2.clone()));
        assert_eq!(s_node_5.right_key(), Some(leaf_3.clone()));
        assert_eq!(s_node_5.parent_key(), Some(node_3.clone()));

        let s_node_9 = storage_map.get(node_9).unwrap();
        assert_eq!(s_node_9.left_key(), Some(leaf_4.clone()));
        assert_eq!(s_node_9.right_key(), Some(leaf_5.clone()));
        assert_eq!(s_node_9.parent_key(), None);

        let s_node_3 = storage_map.get(node_3).unwrap();
        assert_eq!(s_node_3.left_key(), Some(node_1.clone()));
        assert_eq!(s_node_3.right_key(), Some(node_5.clone()));
        assert_eq!(s_node_3.parent_key(), None);
    }

    /*
    #[test]
    fn prove_returns_the_merkle_root_and_proof_set_for_7_leaves() {
        let mut storage_map = StorageMap::<Data, String>::new();
        let mut tree = MerkleTree::new(&mut storage_map);

        let data = &DATA[0..7]; // 7 leaves
        for datum in data.iter() {
            tree.push(datum);
        }

        let mut h = &tree.head;
        while h.is_some() {
            println!("H = {:?}", h);
            h = h.as_ref().unwrap().next()
        }

        //               07
        //              /  \
        //             /    \
        //            /      \
        //           /        \
        //          /          \
        //         /            \
        //       03              11
        //      /  \            /  \
        //     /    \          /    \
        //   01      05       09     \
        //  /  \    /  \     /  \     \
        // 00  02  04  06   08  10    12
        // 00  01  02  03   04  05    06

        let leaf_0 = leaf_data(&data[0]);
        let leaf_1 = leaf_data(&data[1]);
        let leaf_2 = leaf_data(&data[2]);
        let leaf_3 = leaf_data(&data[3]);
        let leaf_4 = leaf_data(&data[4]);
        let leaf_5 = leaf_data(&data[5]);
        let leaf_6 = leaf_data(&data[6]);

        let node_1 = node_data(&leaf_0, &leaf_1);
        let node_5 = node_data(&leaf_2, &leaf_3);
        let node_3 = node_data(&node_1, &node_5);
        let node_9 = node_data(&leaf_4, &leaf_5);
        let node_11 = node_data(&node_9, &leaf_6);
        let node_7 = node_data(&node_3, &node_11);

        {
            let proof = tree.prove(0);
            let root = proof.0;
            let set = proof.1;

            let s_1 = set.get(0).unwrap();
            let s_2 = set.get(1).unwrap();
            let s_3 = set.get(2).unwrap();
            let s_4 = set.get(3).unwrap();

            assert_eq!(root, node_7);
            assert_eq!(s_1, &leaf_0[..]);
            assert_eq!(s_2, &leaf_1[..]);
            assert_eq!(s_3, &node_5[..]);
            assert_eq!(s_4, &node_11[..]);
        }
        {
            let proof = tree.prove(4);
            let root = proof.0;
            let set = proof.1;

            let s_1 = set.get(0).unwrap();
            let s_2 = set.get(1).unwrap();
            let s_3 = set.get(2).unwrap();
            let s_4 = set.get(3).unwrap();

            assert_eq!(root, node_7);
            assert_eq!(s_1, &leaf_4[..]);
            assert_eq!(s_2, &leaf_5[..]);
            assert_eq!(s_3, &leaf_6[..]);
            assert_eq!(s_4, &node_3[..]);
        }
        {
            let proof = tree.prove(5);
            let root = proof.0;
            let set = proof.1;

            let s_1 = set.get(0).unwrap();
            let s_2 = set.get(1).unwrap();
            let s_3 = set.get(2).unwrap();
            let s_4 = set.get(3).unwrap();

            assert_eq!(root, node_7);
            assert_eq!(s_1, &leaf_5[..]);
            assert_eq!(s_2, &leaf_4[..]);
            assert_eq!(s_3, &leaf_6[..]);
            assert_eq!(s_4, &node_3[..]);
        }
        {
            let proof = tree.prove(6);
            let root = proof.0;
            let set = proof.1;

            let s_1 = set.get(0).unwrap();
            let s_2 = set.get(1).unwrap();
            let s_3 = set.get(2).unwrap();

            assert_eq!(root, node_7);
            assert_eq!(s_1, &leaf_6[..]);
            assert_eq!(s_2, &node_9[..]);
            assert_eq!(s_3, &node_3[..]);
        }
    }
    */
}
