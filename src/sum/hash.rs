use crate::digest::Digest;
use std::convert::TryFrom;

const NODE: [u8; 1] = [0x01];
const LEAF: [u8; 1] = [0x00];

type Data = [u8; 32];

// Merkle Tree hash of an empty list
// MTH({}) = Hash()
pub fn empty_sum<D: Digest>() -> Data {
    let hash = D::new();
    let data = hash.finalize();

    <Data>::try_from(data.as_slice()).unwrap()
}

// Merkle tree hash of an n-element list D[n]
// MTH(D[n]) = Hash(0x01 || MTH(D[0:k]) || MTH(D[k:n])
pub fn node_sum<D: Digest>(lhs_fee: u64, lhs_data: &[u8], rhs_fee: u64, rhs_data: &[u8]) -> Data {
    let mut hash = D::new();

    hash.update(&NODE);
    hash.update(lhs_fee.to_be_bytes());
    hash.update(&lhs_data);
    hash.update(rhs_fee.to_be_bytes());
    hash.update(&rhs_data);
    let data = hash.finalize();

    <Data>::try_from(data.as_slice()).unwrap()
}

// Merkle tree hash of a list with one entry
// MTH({d(0)}) = Hash(0x00 || d(0))
pub fn leaf_sum<D: Digest>(data: &[u8]) -> Data {
    let mut hash = D::new();

    hash.update(&LEAF);
    hash.update(&data);
    let data = hash.finalize();

    <Data>::try_from(data.as_slice()).unwrap()
}
