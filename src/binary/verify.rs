// use crate::digest2::digest2::Digest;
// use crate::binary::merkle_hash::MerkleHash;
// use crate::test_hash::simple_hash::SimpleHash;
//
// const BRANCH: [u8; 1] = [0x01];
// const LEAF: [u8; 1] = [0x00];
//
// type H = [u8];
//
// pub fn verify<D: Digest>(h: &mut MerkleHash<D>, root: &H, proof_set: &[&H], proof_index: u32, num_leaves: u32) -> bool {
//     let mut height = 0;
//
//     h.update(&LEAF);
//     h.update(&proof_set[0]);
//     h.finalize();
//     height += 1;
//
//     h.compare(&root)
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     type MH = MerkleHash<SimpleHash>;
//
//     #[test]
//     fn test_it() {
//         let mut h = MH::new();
//         let root = [1u8; 32];
//         let proof_set = [
//             "hello".as_bytes(),
//             "world".as_bytes()
//         ];
//         // let v = verify(&mut h, &root, &proof_set);
//         //
//         // let b = h.buffer();
//         // println!("{:?}", b);
//         //
//         // assert!(v);
//     }
// }
