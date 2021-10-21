mod node;
mod path_iterator;
mod position;
mod storage_map;

pub use node::{Node, ParentNode};
pub use path_iterator::IntoPathIterator;
pub use position::Position;
pub use storage_map::{StorageError, StorageMap};

pub type Bytes1 = [u8; 1];
pub type Bytes2 = [u8; 2];
pub type Bytes4 = [u8; 4];
pub type Bytes8 = [u8; 8];
pub type Bytes16 = [u8; 16];
pub type Bytes32 = [u8; 32];

pub trait MSB {
    fn get_bit_at_index_from_msb(&self, index: usize) -> u8;
}

use std::mem::size_of;

macro_rules! impl_msb_for {
    ($t: ty) => {
        impl MSB for $t {
            fn get_bit_at_index_from_msb(&self, index: usize) -> u8 {
                // The byte that contains the requested bit
                let byte_index = index / 8;
                assert!(byte_index < size_of::<$t>());
                let byte = self[byte_index];

                // The bit within the byte
                let byte_bit_index = index % 8;
                let shift = 1 << (8 - 1 - byte_bit_index);
                let bit = (byte & shift) != 0;

                bit as u8
            }
        }
    };
}

impl_msb_for!(Bytes1);
impl_msb_for!(Bytes2);
impl_msb_for!(Bytes4);
impl_msb_for!(Bytes8);
impl_msb_for!(Bytes16);
impl_msb_for!(Bytes32);

#[cfg(test)]
mod test {
    use super::{Bytes32, MSB};
    use crate::common::{Bytes1, Bytes2, Bytes4, Bytes8};
    use std::mem::size_of;

    #[test]
    fn test_msb_for_bytes_1() {
        const NUM_BITS: usize = size_of::<Bytes1>() * 8;

        let bytes: Bytes1 = [0b10101010];
        let expected_n = u8::from_be_bytes(bytes);

        let mut n = 0;
        for i in 0..NUM_BITS {
            let bit = bytes.get_bit_at_index_from_msb(i);
            let shift = bit << (NUM_BITS - 1 - i);
            n |= shift;
        }

        assert_eq!(n, expected_n);
    }

    #[test]
    fn test_msb_for_bytes_2() {
        const NUM_BITS: usize = size_of::<Bytes2>() * 8;

        let bytes: Bytes2 = [0b10101010, 0b10101010];
        let expected_n = u16::from_be_bytes(bytes);

        let mut n = 0;
        for i in 0..NUM_BITS {
            let bit = bytes.get_bit_at_index_from_msb(i) as u16;
            let shift = bit << (NUM_BITS - 1 - i);
            n |= shift;
        }

        assert_eq!(n, expected_n);
    }

    #[test]
    fn test_msb_for_bytes_4() {
        const NUM_BITS: usize = size_of::<Bytes4>() * 8;

        let bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];
        let expected_n = u32::from_be_bytes(bytes);

        let mut n = 0;
        for i in 0..NUM_BITS {
            let bit = bytes.get_bit_at_index_from_msb(i) as u32;
            let shift = bit << (NUM_BITS - 1 - i);
            n |= shift;
        }

        assert_eq!(n, expected_n);
    }

    #[test]
    fn test_msb_for_bytes_8() {
        const NUM_BITS: usize = size_of::<Bytes8>() * 8;

        let bytes: Bytes8 = [
            0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b10101010,
            0b10101010,
        ];
        let expected_n = u64::from_be_bytes(bytes);

        let mut n = 0;
        for i in 0..NUM_BITS {
            let bit = bytes.get_bit_at_index_from_msb(i) as u64;
            let shift = bit << (NUM_BITS - 1 - i);
            n |= shift;
        }

        assert_eq!(n, expected_n);
    }
}
