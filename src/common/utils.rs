use std::convert::TryFrom;

pub fn smear_ones_u64(v: u64) -> u64 {
    let mut n = v;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n
}

pub fn msb_index_from_right(v: u64) -> u32 {
    let mask = smear_ones_u64(v);
    let index = (mask + 1).trailing_zeros();
    index
}

#[cfg(test)]
mod test {
    use crate::common::utils::smear_ones_u64;

    #[test]
    fn test_smear_ones_u64() {
        let n = 1u64 << 7;
        assert_eq!(smear_ones_u64(n), u8::MAX as u64);

        let n = 1u64 << 15;
        assert_eq!(smear_ones_u64(n), u16::MAX as u64);

        let n = 1u64 << 31;
        assert_eq!(smear_ones_u64(n), u32::MAX as u64);

        let n = 1u64 << 63;
        assert_eq!(smear_ones_u64(n), u64::MAX as u64);
    }
}
