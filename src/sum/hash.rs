use digest::Digest;
use generic_array::GenericArray;
use lazy_static::lazy_static;
use sha2::Sha256 as Hash;

pub type Data = GenericArray<u8, <Hash as Digest>::OutputSize>;

const NODE: u8 = 0x01;
const LEAF: u8 = 0x00;

lazy_static! {
    static ref EMPTY_SUM: Data = Hash::new().finalize();
}

// Merkle Tree hash of an empty list
// MTH({}) = Hash()
pub fn empty_sum() -> &'static Data {
    &*EMPTY_SUM
}

// Merkle tree hash of an n-element list D[n]
// MTH(D[n]) = Hash(0x01 || LHS fee || MTH(D[0:k]) || RHS fee || MTH(D[k:n])
pub fn node_sum(lhs_fee: u64, lhs_data: &[u8], rhs_fee: u64, rhs_data: &[u8]) -> Data {
    let mut hash = Hash::new();
    hash.update(&[NODE]);
    hash.update(lhs_fee.to_be_bytes());
    hash.update(&lhs_data);
    hash.update(rhs_fee.to_be_bytes());
    hash.update(&rhs_data);
    hash.finalize()
}

// Merkle tree hash of a list with one entry
// MTH({d(0)}) = Hash(0x00 || d(0))
pub fn leaf_sum(data: &[u8]) -> Data {
    let mut hash = Hash::new();
    hash.update(&[LEAF]);
    hash.update(&data);
    hash.finalize()
}
