use digest::Digest;
use generic_array::GenericArray;
use hex;
use lazy_static::lazy_static;
use sha2::Sha256;
use std::fmt::Formatter;

pub type Hash = Sha256;

type HashArray = GenericArray<u8, <Hash as Digest>::OutputSize>;

#[derive(Clone, Eq, PartialEq, std::hash::Hash, Debug)]
pub struct Data(HashArray);

impl Data {
    pub fn new(data: HashArray) -> Self {
        Self(data)
    }

    pub fn data(&self) -> &HashArray {
        return &self.0;
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data(0x{})", hex::encode(self.data()))
    }
}

const NODE: u8 = 0x01;
const LEAF: u8 = 0x00;

lazy_static! {
    static ref EMPTY_SUM: Data = Data(Hash::new().finalize());
}

// Merkle Tree hash of an empty list
// MTH({}) = Hash()
pub fn empty_sum() -> &'static Data {
    &*EMPTY_SUM
}

// Merkle tree hash of an n-element list D[n]
// MTH(D[n]) = Hash(0x01 || MTH(D[0:k]) || MTH(D[k:n])
pub fn node_sum(lhs_data: &[u8], rhs_data: &[u8]) -> Data {
    let mut hash = Hash::new();
    hash.update(&[NODE]);
    hash.update(&lhs_data);
    hash.update(&rhs_data);
    Data(hash.finalize())
}

// Merkle tree hash of a list with one entry
// MTH({d(0)}) = Hash(0x00 || d(0))
pub fn leaf_sum(data: &[u8]) -> Data {
    let mut hash = Hash::new();
    hash.update(&[LEAF]);
    hash.update(&data);
    Data(hash.finalize())
}
