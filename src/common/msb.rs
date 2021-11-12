use crate::common::{Bytes1, Bytes2, Bytes4, Bytes8};
use crate::common::{Bytes16, Bytes32};
use std::mem::size_of;

pub trait MSB {
    fn get_bit_at_index_from_msb(&self, index: usize) -> u8;
    fn common_prefix_count(&self, other: &Self) -> usize;
}

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
                let shift = 1 << (7 - byte_bit_index);
                let bit = (byte & shift) != 0;

                bit as u8
            }

            fn common_prefix_count(&self, other: &Self) -> usize {
                const SIZE_BITS: usize = size_of::<$t>() * 8;

                let mut count = 0;
                for i in 0..SIZE_BITS {
                    let lhs_bit = self.get_bit_at_index_from_msb(i);
                    let rhs_bit = other.get_bit_at_index_from_msb(i);
                    if lhs_bit == rhs_bit {
                        count += 1;
                    } else {
                        break;
                    }
                }
                count
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
    use crate::common::{Bytes1, Bytes2, Bytes4, Bytes8, MSB};
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

    #[test]
    #[should_panic]
    fn test_get_bit_at_index_from_msb_panics_for_index_out_of_bounds() {
        let bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];

        // Should panic; acceptable inputs for Bytes4 are in [0, 31]
        bytes.get_bit_at_index_from_msb(32);
    }

    #[test]
    fn test_common_prefix_count_returns_count_of_common_bits_when_all_bits_match() {
        let lhs_bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];
        let rhs_bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];
        let common_prefix_count = lhs_bytes.common_prefix_count(&rhs_bytes);

        assert_eq!(common_prefix_count, 4 * 8);
    }

    #[test]
    fn test_common_prefix_count_returns_count_of_common_bits_when_some_bits_match() {
        let lhs_bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];
        let rhs_bytes: Bytes4 = [0b10101010, 0b10101010, !0b10101010, 0b10101010];
        let common_prefix_count = lhs_bytes.common_prefix_count(&rhs_bytes);

        assert_eq!(common_prefix_count, 2 * 8);
    }

    #[test]
    fn test_common_prefix_count_returns_0_when_the_first_bits_are_different() {
        let lhs_bytes: Bytes4 = [0b10101010, 0b10101010, 0b10101010, 0b10101010];
        let rhs_bytes: Bytes4 = [0b00101010, 0b10101010, 0b10101010, 0b10101010];
        let common_prefix_count = lhs_bytes.common_prefix_count(&rhs_bytes);

        assert_eq!(common_prefix_count, 0);
    }
}
