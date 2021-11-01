pub(crate) use digest::Digest;

use lazy_static::lazy_static;
use sha2::Sha256;
use std::convert::TryInto;

use crate::common::{Bytes32, LEAF, NODE};

pub(crate) type Hash = Sha256;

lazy_static! {
    static ref EMPTY_SUM: Bytes32 = Hash::new().finalize().try_into().unwrap();
    static ref ZERO_SUM: Bytes32 = ['\0' as u8; 32];
}

// Merkle Tree hash of an empty list
// MTH({}) = Hash()
pub fn empty_sum() -> &'static Bytes32 {
    &*EMPTY_SUM
}

pub fn zero_sum() -> &'static Bytes32 {
    &*ZERO_SUM
}

pub fn sum(data: &[u8]) -> Bytes32 {
    let mut hash = Hash::new();
    hash.update(&data);
    hash.finalize().try_into().unwrap()
}

// Merkle tree hash of an n-element list D[n]
// MTH(D[n]) = Hash(0x01 || MTH(D[0:k]) || MTH(D[k:n])
pub fn node_sum(lhs_data: &[u8], rhs_data: &[u8]) -> Bytes32 {
    let mut hash = Hash::new();
    hash.update(&[NODE]);
    hash.update(&lhs_data);
    hash.update(&rhs_data);
    hash.finalize().try_into().unwrap()
}

// Merkle tree hash of a list with one entry
// MTH({d(0)}) = Hash(0x00 || d(0))
pub fn leaf_sum(data: &[u8]) -> Bytes32 {
    let mut hash = Hash::new();
    hash.update(&[LEAF]);
    hash.update(&data);
    hash.finalize().try_into().unwrap()
}
