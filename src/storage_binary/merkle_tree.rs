// use crate::storage_binary::hash::{empty_sum, leaf_sum, node_sum, Data};
// use crate::storage_binary::node::Node;
// use crate::common::storage::Storage;
// use crate::common::{position::Position};
// use crate::proof_set::ProofSet;
// use digest::generic_array::GenericArray;
//
// type DataNode = Node<Data>;
//
// pub struct MerkleTree<'storage> {
//     storage: &'storage mut dyn Storage,
//     head: Option<Box<DataNode>>,
//     leaves_count: u64,
// }
//
// impl<'storage> MerkleTree<'storage> {
//     pub fn new(storage: &'storage mut dyn Storage) -> Self {
//         let mut tree = Self {
//             storage,
//             head: None,
//             leaves_count: 0,
//         };
//
//         tree.initialize();
//         tree
//     }
//
//     pub fn root(&self) -> Data {
//         match self.head() {
//             None => empty_sum().clone(),
//             Some(ref head) => {
//                 let mut current = head.clone();
//                 while current.next().is_some() {
//                     let mut node = current;
//                     let mut next_node = node.take_next().unwrap();
//                     current = Self::join_subtrees(&mut next_node, &node)
//                 }
//                 current.data().clone()
//             }
//         }
//     }
//
//     pub fn leaves_count(&self) -> u64 {
//         self.leaves_count
//     }
//
//     pub fn prove(&mut self, proof_index: u64) -> (Data, ProofSet) {
//         let mut proof_set = ProofSet::new();
//
//         if self.head().is_none() {
//             return (self.root(), proof_set);
//         }
//
//         let mut current = self.head().clone().unwrap();
//         while current.next().is_some() {
//             let mut node = current;
//             let mut next_node = node.take_next().unwrap();
//             current = Self::join_subtrees(&mut next_node, &node);
//
//             let position = current.position();
//             let data = current.data();
//             self.persist_node(position, data);
//         }
//         let max_height = current.height();
//
//
//         let max_position = Position::from_leaf_index(self.leaves_count() - 1);
//         let proof_position = Position::from_leaf_index(proof_index);
//
//         {
//             // Push the leaf at the proof index
//             let node = self.storage.read_node(proof_position).unwrap();
//             println!("pushing proof: {:?}", proof_position);
//             proof_set.push(node.data());
//         }
//
//         let mut current = self.head().clone().unwrap();
//         if current.next().is_some() && proof_position.is_descendant_of(current.position()) {
//             println!("{:?} is descendant of {:?}", proof_position, current.position());
//             current = current.take_next().unwrap();
//         }
//
//         let position_iter = proof_position
//             .iter_uncle()
//             .take_while(|position| position.height() < max_height);
//         for pos in position_iter {
//             if pos.in_order_index() <= max_position.in_order_index() {
//                 let node = self.storage.read_node(pos).unwrap();
//                 println!("pushing pos: {:?}", pos);
//                 proof_set.push(node.data());
//             } else {
//                 // Exited the tree; there is at least 1 head node
//                 // Join all heads with height lower than pos
//                 println!("exiting tree {:?}", pos);
//                 while current.next().is_some() && current.next_height().unwrap() < pos.height() {
//                     let mut node = current;
//                     let mut next_node = node.take_next().unwrap();
//                     print!("joining {:?} and {:?}", node.position(), next_node.position());
//                     current = Self::join_subtrees(&mut next_node, &node);
//                     println!(" = {:?}", current.position());
//                 }
//                 if current.height() < pos.height() {
//                     println!("pushing current pos: {:?}", current.position());
//                     proof_set.push(current.data());
//                     current = current.take_next().unwrap();
//                 }
//             }
//         }
//
//         println!();
//         println!();
//
//         (self.root(), proof_set)
//     }
//
//     pub fn push(&mut self, data: &[u8]) {
//         let leaf_sum = leaf_sum(data);
//
//         // Get leaf position from current leaves count:
//         // The position is determined as the in-order position in the binary tree.
//         // The leaf's position will be the next even number, starting at 0.
//         let position = Position::from_leaf_index(self.leaves_count());
//         self.add(position, &leaf_sum);
//
//         // Persist the new leaf
//         self.persist_node(position, &leaf_sum);
//     }
//
//     //
//     // PRIVATE
//     //
//
//     fn initialize(&mut self) {
//         for node in self.storage.get_all_nodes() {
//             let data = GenericArray::from_slice(node.data());
//             self.add(node.key(), data);
//         }
//     }
//
//     fn add(&mut self, position: Position, data: &Data) {
//         let node = Self::create_node(self.head.take(), position, data.clone());
//         self.head = Some(node);
//
//         self.join_all_subtrees();
//
//         self.leaves_count += 1;
//     }
//
//     fn head(&self) -> &Option<Box<DataNode>> {
//         &self.head
//     }
//
//     fn join_all_subtrees(&mut self) {
//         loop {
//             let head = self.head.as_ref().unwrap();
//             if !(head.next().is_some() && head.height() == head.next_height().unwrap()) {
//                 break;
//             }
//
//             // Merge the two front nodes of the list into a single node
//             let mut node = self.head.take().unwrap();
//             let next_node = node.take_next().unwrap();
//             let joined_node = Self::join_subtrees(&mut next_node.clone(), &node.clone());
//
//             // Persist the joined node
//             let position = joined_node.position();
//             let data = joined_node.data();
//             self.persist_node(position, data);
//
//             self.head = Some(joined_node);
//         }
//     }
//
//     fn join_subtrees(a: &mut DataNode, b: &DataNode) -> Box<DataNode> {
//         let next = a.take_next();
//         let position = a.position().parent();
//         let data = node_sum(a.data(), b.data());
//         Self::create_node(next, position, data.clone())
//     }
//
//     fn create_node(next: Option<Box<DataNode>>, position: Position, data: Data) -> Box<DataNode> {
//         let node = DataNode::new(next, position, data);
//         Box::new(node)
//     }
//
//     fn persist_node(&mut self, position: Position, data: &Data) {
//         self.storage.create_node(position, data);
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::storage_binary::hash::Hash;
//     use crate::common::storage_map::StorageMap;
//     use digest::Digest;
//
//     const NODE: u8 = 0x01;
//     const LEAF: u8 = 0x00;
//
//     fn empty_data() -> Data {
//         let hash = Hash::new();
//         hash.finalize()
//     }
//     fn leaf_data(data: &[u8]) -> Data {
//         let mut hash = Hash::new();
//         hash.update(&[LEAF]);
//         hash.update(&data);
//         hash.finalize()
//     }
//     fn node_data(lhs_data: &[u8], rhs_data: &[u8]) -> Data {
//         let mut hash = Hash::new();
//         hash.update(&[NODE]);
//         hash.update(&lhs_data);
//         hash.update(&rhs_data);
//         hash.finalize()
//     }
//
//     const DATA: [&[u8]; 20] = [
//         "Frankly, my dear, I don't give a damn.".as_bytes(),
//         "I'm going to make him an offer he can't refuse".as_bytes(),
//         "Toto, I've got a feeling we're not in Kansas anymore.".as_bytes(),
//         "Here's looking at you, kid.".as_bytes(),
//         "Go ahead, make my day.".as_bytes(),
//         "May the Force be with you.".as_bytes(),
//         "You talking to me?".as_bytes(),
//         "What we've got here is failure to communicate.".as_bytes(),
//         "I love the smell of napalm in the morning.".as_bytes(),
//         "Love means never having to say you're sorry.".as_bytes(),
//
//         "Frankly, my dear, I don't give a damn.".as_bytes(),
//         "I'm going to make him an offer he can't refuse".as_bytes(),
//         "Toto, I've got a feeling we're not in Kansas anymore.".as_bytes(),
//         "Here's looking at you, kid.".as_bytes(),
//         "Go ahead, make my day.".as_bytes(),
//         "May the Force be with you.".as_bytes(),
//         "You talking to me?".as_bytes(),
//         "What we've got here is failure to communicate.".as_bytes(),
//         "I love the smell of napalm in the morning.".as_bytes(),
//         "Love means never having to say you're sorry.".as_bytes(),
//     ];
//
//     #[test]
//     fn root_returns_the_hash_of_the_empty_string_when_no_leaves_are_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mt = MerkleTree::new(&mut storage_map);
//
//         let root = mt.root();
//
//         let expected = empty_data();
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn root_returns_the_hash_of_the_leaf_when_one_leaf_is_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..1]; // 1 leaf
//         mt.push(&data[0]);
//
//         let root = mt.root();
//
//         let expected = leaf_data(&data[0]);
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn root_returns_the_hash_of_the_head_when_2_leaves_are_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..2]; // 2 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let root = mt.root();
//
//         //   N1
//         //  /  \
//         // L1  L2
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let node_1 = node_data(&leaf_1, &leaf_2);
//
//         let expected = node_1;
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn root_returns_the_hash_of_the_head_when_4_leaves_are_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..4]; // 4 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let root = mt.root();
//
//         //       N3
//         //      /  \
//         //     /    \
//         //   N1      N2
//         //  /  \    /  \
//         // L1  L2  L3  L4
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//         let leaf_4 = leaf_data(&data[3]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&leaf_3, &leaf_4);
//         let node_3 = node_data(&node_1, &node_2);
//
//         let expected = node_3;
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn root_returns_the_hash_of_the_head_when_5_leaves_are_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..5]; // 5 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let root = mt.root();
//
//         //          N4
//         //         /  \
//         //       N3    \
//         //      /  \    \
//         //     /    \    \
//         //   N1      N2   \
//         //  /  \    /  \   \
//         // L1  L2  L3  L4  L5
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//         let leaf_4 = leaf_data(&data[3]);
//         let leaf_5 = leaf_data(&data[4]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&leaf_3, &leaf_4);
//         let node_3 = node_data(&node_1, &node_2);
//         let node_4 = node_data(&node_3, &leaf_5);
//
//         let expected = node_4;
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn root_returns_the_hash_of_the_head_when_7_leaves_are_pushed() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..7]; // 7 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//         let root = mt.root();
//
//         //              N6
//         //          /        \
//         //         /          \
//         //       N4            N5
//         //      /  \           /\
//         //     /    \         /  \
//         //   N1      N2      N3   \
//         //  /  \    /  \    /  \   \
//         // L1  L2  L3  L4  L5  L6  L7
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//         let leaf_4 = leaf_data(&data[3]);
//         let leaf_5 = leaf_data(&data[4]);
//         let leaf_6 = leaf_data(&data[5]);
//         let leaf_7 = leaf_data(&data[6]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&leaf_3, &leaf_4);
//         let node_3 = node_data(&leaf_5, &leaf_6);
//         let node_4 = node_data(&node_1, &node_2);
//         let node_5 = node_data(&node_3, &leaf_7);
//         let node_6 = node_data(&node_4, &node_5);
//
//         let expected = node_6;
//         assert_eq!(root, expected);
//     }
//
//     #[test]
//     fn leaves_count_returns_the_number_of_leaves_pushed_to_the_tree() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..4];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         assert_eq!(mt.leaves_count(), data.len() as u64);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..4]; // 4 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let proof = mt.prove(0);
//         let root = proof.0;
//         let set = proof.1;
//
//         //       N3
//         //      /  \
//         //     /    \
//         //   N1      N2
//         //  /  \    /  \
//         // L1  L2  L3  L4
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//         let leaf_4 = leaf_data(&data[3]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&leaf_3, &leaf_4);
//         let node_3 = node_data(&node_1, &node_2);
//
//         let s_1 = set.get(0).unwrap();
//         let s_2 = set.get(1).unwrap();
//         let s_3 = set.get(2).unwrap();
//
//         assert_eq!(root, node_3);
//         assert_eq!(s_1, &leaf_1[..]);
//         assert_eq!(s_2, &leaf_2[..]);
//         assert_eq!(s_3, &node_2[..]);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_in_a_mmr() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..3]; // 3 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         //     N2
//         //    /  \
//         //   N1   \
//         //  /  \   \
//         // L1  L2  L3
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&node_1, &leaf_3);
//
//         {
//             let proof = mt.prove(0);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//
//             assert_eq!(root, node_2);
//             assert_eq!(s_1, &leaf_1[..]);
//             assert_eq!(s_2, &leaf_2[..]);
//             assert_eq!(s_3, &leaf_3[..]);
//         }
//
//         {
//             let proof = mt.prove(1);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//
//             assert_eq!(root, node_2);
//             assert_eq!(s_1, &leaf_2[..]);
//             assert_eq!(s_2, &leaf_1[..]);
//             assert_eq!(s_3, &leaf_3[..]);
//         }
//
//         {
//             let proof = mt.prove(2);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//
//             assert_eq!(root, node_2);
//             assert_eq!(s_1, &leaf_3[..]);
//             assert_eq!(s_2, &node_1[..]);
//         }
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_left_of_the_root() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..5]; // 5 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         //          N4
//         //         /  \
//         //       N3    \
//         //      /  \    \
//         //     /    \    \
//         //   N1      N2   \
//         //  /  \    /  \   \
//         // L1  L2  L3  L4  L5
//
//         let leaf_1 = leaf_data(&data[0]);
//         let leaf_2 = leaf_data(&data[1]);
//         let leaf_3 = leaf_data(&data[2]);
//         let leaf_4 = leaf_data(&data[3]);
//         let leaf_5 = leaf_data(&data[4]);
//
//         let node_1 = node_data(&leaf_1, &leaf_2);
//         let node_2 = node_data(&leaf_3, &leaf_4);
//         let node_3 = node_data(&node_1, &node_2);
//         let node_4 = node_data(&node_3, &leaf_5);
//
//         {
//             let proof = mt.prove(0);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_4);
//             assert_eq!(s_1, &leaf_1[..]);
//             assert_eq!(s_2, &leaf_2[..]);
//             assert_eq!(s_3, &node_2[..]);
//             assert_eq!(s_4, &leaf_5[..]);
//         }
//
//         {
//             let proof = mt.prove(2);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_4);
//             assert_eq!(s_1, &leaf_3[..]);
//             assert_eq!(s_2, &leaf_4[..]);
//             assert_eq!(s_3, &node_1[..]);
//             assert_eq!(s_4, &leaf_5[..]);
//         }
//
//         {
//             let proof = mt.prove(4);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//
//             assert_eq!(root, node_4);
//             assert_eq!(s_1, &leaf_5[..]);
//             assert_eq!(s_2, &node_3[..]);
//         }
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_7_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..7]; // 7 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         //               07
//         //              /  \
//         //             /    \
//         //            /      \
//         //           /        \
//         //          /          \
//         //         /            \
//         //       03              11
//         //      /  \            /  \
//         //     /    \          /    \
//         //   01      05       09     \
//         //  /  \    /  \     /  \     \
//         // 00  02  04  06   08  10    12
//         // 00  01  02  03   04  05    06
//
//         let leaf_0 = leaf_data(&data[0]);
//         let leaf_1 = leaf_data(&data[1]);
//         let leaf_2 = leaf_data(&data[2]);
//         let leaf_3 = leaf_data(&data[3]);
//         let leaf_4 = leaf_data(&data[4]);
//         let leaf_5 = leaf_data(&data[5]);
//         let leaf_6 = leaf_data(&data[6]);
//
//         let node_1 = node_data(&leaf_0, &leaf_1);
//         let node_5 = node_data(&leaf_2, &leaf_3);
//         let node_3 = node_data(&node_1, &node_5);
//         let node_9 = node_data(&leaf_4, &leaf_5);
//         let node_11 = node_data(&node_9, &leaf_6);
//         let node_7 = node_data(&node_3, &node_11);
//
//         {
//             let proof = mt.prove(0);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_7);
//             assert_eq!(s_1, &leaf_0[..]);
//             assert_eq!(s_2, &leaf_1[..]);
//             assert_eq!(s_3, &node_5[..]);
//             assert_eq!(s_4, &node_11[..]);
//         }
//
//         {
//             let proof = mt.prove(5);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_7);
//             assert_eq!(s_1, &leaf_5[..]);
//             assert_eq!(s_2, &leaf_4[..]);
//             assert_eq!(s_3, &leaf_6[..]);
//             assert_eq!(s_4, &node_3[..]);
//         }
//
//         {
//             let proof = mt.prove(4);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_7);
//             assert_eq!(s_1, &leaf_4[..]);
//             assert_eq!(s_2, &leaf_5[..]);
//             assert_eq!(s_3, &leaf_6[..]);
//             assert_eq!(s_4, &node_3[..]);
//         }
//
//         {
//             let proof = mt.prove(6);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//
//             assert_eq!(root, node_7);
//             assert_eq!(s_1, &leaf_6[..]);
//             assert_eq!(s_2, &node_9[..]);
//             assert_eq!(s_3, &node_3[..]);
//         }
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_15_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..15]; // 15 leaves
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let leaf_0 = leaf_data(&data[0]);
//         let leaf_1 = leaf_data(&data[1]);
//         let leaf_2 = leaf_data(&data[2]);
//         let leaf_3 = leaf_data(&data[3]);
//         let leaf_4 = leaf_data(&data[4]);
//         let leaf_5 = leaf_data(&data[5]);
//         let leaf_6 = leaf_data(&data[6]);
//         let leaf_7 = leaf_data(&data[7]);
//         let leaf_8 = leaf_data(&data[8]);
//         let leaf_9 = leaf_data(&data[9]);
//         let leaf_10 = leaf_data(&data[10]);
//         let leaf_11 = leaf_data(&data[11]);
//         let leaf_12 = leaf_data(&data[12]);
//         let leaf_13 = leaf_data(&data[13]);
//         let leaf_14 = leaf_data(&data[14]);
//
//         let node_1 = node_data(&leaf_0, &leaf_1);
//         let node_5 = node_data(&leaf_2, &leaf_3);
//         let node_9 = node_data(&leaf_4, &leaf_5);
//         let node_13 = node_data(&leaf_6, &leaf_7);
//         let node_17 = node_data(&leaf_8, &leaf_9);
//         let node_21 = node_data(&leaf_10, &leaf_11);
//         let node_25 = node_data(&leaf_12, &leaf_13);
//
//         let node_3 = node_data(&node_1, &node_5);
//         let node_11 = node_data(&node_9, &node_13);
//         let node_19 = node_data(&node_17, &node_21);
//
//         let node_7 = node_data(&node_3, &node_11);
//
//         let node_27 = node_data(&node_25, &leaf_14);
//         let node_23 = node_data(&node_19, &node_27);
//         let node_15 = node_data(&node_7, &node_23);
//
//         {
//             let proof = mt.prove(0);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//             let s_5 = set.get(4).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_0[..]);
//             assert_eq!(s_2, &leaf_1[..]);
//             assert_eq!(s_3, &node_5[..]);
//             assert_eq!(s_4, &node_11[..]);
//             assert_eq!(s_5, &node_23[..]);
//         }
//
//         {
//             let proof = mt.prove(8);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//             let s_5 = set.get(4).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_8[..]);
//             assert_eq!(s_2, &leaf_9[..]);
//             assert_eq!(s_3, &node_21[..]);
//             assert_eq!(s_4, &node_27[..]);
//             assert_eq!(s_5, &node_7[..]);
//         }
//
//         {
//             let proof = mt.prove(12);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//             let s_5 = set.get(4).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_12[..]);
//             assert_eq!(s_2, &leaf_13[..]);
//             assert_eq!(s_3, &leaf_14[..]);
//             assert_eq!(s_4, &node_19[..]);
//             assert_eq!(s_5, &node_7[..]);
//         }
//
//         {
//             let proof = mt.prove(14);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_14[..]);
//             assert_eq!(s_2, &node_25[..]);
//             assert_eq!(s_3, &node_19[..]);
//             assert_eq!(s_4, &node_7[..]);
//         }
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_17_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..17];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         mt.prove(16);
//         println!();
//
//         mt.prove(0);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_18_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..18];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         mt.prove(16);
//         println!();
//
//         mt.prove(0);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_19_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..19];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         mt.prove(16);
//         println!();
//
//         mt.prove(0);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_20_leaves() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..20];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         mt.prove(19);
//         println!();
//
//         mt.prove(0);
//     }
//
//     #[test]
//     fn prove_returns_the_merkle_root_and_proof_set_for_the_given_proof_index_left_of_the_root_4() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let data = &DATA[0..14];
//         for datum in data.iter() {
//             mt.push(datum);
//         }
//
//         let leaf_0 = leaf_data(&data[0]);
//         let leaf_1 = leaf_data(&data[1]);
//         let leaf_2 = leaf_data(&data[2]);
//         let leaf_3 = leaf_data(&data[3]);
//         let leaf_4 = leaf_data(&data[4]);
//         let leaf_5 = leaf_data(&data[5]);
//         let leaf_6 = leaf_data(&data[6]);
//         let leaf_7 = leaf_data(&data[7]);
//         let leaf_8 = leaf_data(&data[8]);
//         let leaf_9 = leaf_data(&data[9]);
//         let leaf_10 = leaf_data(&data[10]);
//         let leaf_11 = leaf_data(&data[11]);
//         let leaf_12 = leaf_data(&data[12]);
//         let leaf_13 = leaf_data(&data[13]);
//
//         let node_1 = node_data(&leaf_0, &leaf_1);
//         let node_5 = node_data(&leaf_2, &leaf_3);
//         let node_9 = node_data(&leaf_4, &leaf_5);
//         let node_13 = node_data(&leaf_6, &leaf_7);
//         let node_17 = node_data(&leaf_8, &leaf_9);
//         let node_21 = node_data(&leaf_10, &leaf_11);
//         let node_25 = node_data(&leaf_12, &leaf_13);
//
//         let node_3 = node_data(&node_1, &node_5);
//         let node_11 = node_data(&node_9, &node_13);
//         let node_19 = node_data(&node_17, &node_21);
//
//         let node_7 = node_data(&node_3, &node_11);
//
//         let node_23 = node_data(&node_19, &node_25);
//         let node_15 = node_data(&node_7, &node_23);
//
//         {
//             let proof = mt.prove(0);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//             let s_5 = set.get(4).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_0[..]);
//             assert_eq!(s_2, &leaf_1[..]);
//             assert_eq!(s_3, &node_5[..]);
//             assert_eq!(s_4, &node_11[..]);
//             assert_eq!(s_5, &node_23[..]);
//         }
//
//         {
//             let proof = mt.prove(8);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//             let s_5 = set.get(4).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_8[..]);
//             assert_eq!(s_2, &leaf_9[..]);
//             assert_eq!(s_3, &node_21[..]);
//             assert_eq!(s_4, &node_25[..]);
//             assert_eq!(s_5, &node_7[..]);
//         }
//
//         {
//             let proof = mt.prove(12);
//             let root = proof.0;
//             let set = proof.1;
//
//             let s_1 = set.get(0).unwrap();
//             let s_2 = set.get(1).unwrap();
//             let s_3 = set.get(2).unwrap();
//             let s_4 = set.get(3).unwrap();
//
//             assert_eq!(root, node_15);
//             assert_eq!(s_1, &leaf_12[..]);
//             assert_eq!(s_2, &leaf_13[..]);
//             assert_eq!(s_3, &node_19[..]);
//             assert_eq!(s_4, &node_7[..]);
//         }
//
//         // {
//         //     let proof = mt.prove(14);
//         //     let root = proof.0;
//         //     let set = proof.1;
//         //
//         //     let s_1 = set.get(0).unwrap();
//         //     let s_2 = set.get(1).unwrap();
//         //     let s_3 = set.get(2).unwrap();
//         //     let s_4 = set.get(3).unwrap();
//         //
//         //     assert_eq!(root, node_15);
//         //     assert_eq!(s_1, &leaf_14[..]);
//         //     assert_eq!(s_2, &node_25[..]);
//         //     assert_eq!(s_3, &node_19[..]);
//         //     assert_eq!(s_4, &node_7[..]);
//         // }
//     }
//
//
//     #[test]
//     fn prove_returns_the_root_of_the_empty_merkle_tree_when_no_leaves_are_added() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let proof = mt.prove(0);
//         let root = proof.0;
//
//         let expected_root = empty_data();
//         assert_eq!(root, expected_root);
//     }
//
//     #[test]
//     fn prove_returns_an_empty_proof_set_when_no_leaves_are_added() {
//         let mut storage_map = StorageMap::new();
//         let mut mt = MerkleTree::new(&mut storage_map);
//
//         let proof = mt.prove(0);
//         let set = proof.1;
//
//         assert_eq!(set.len(), 0);
//     }
// }