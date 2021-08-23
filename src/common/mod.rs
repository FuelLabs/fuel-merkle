pub mod position;
pub mod storage;
pub mod storage_map;

pub fn msb_u64(mut n: u64) -> u64 {
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n += 1;
    n >> 1
}

#[cfg(test)]
mod test {
    use crate::common::msb_u64;

    #[test]
    fn test_msb() {
        assert_eq!(msb_u64(0), 0);
        assert_eq!(msb_u64(1), 1);
        assert_eq!(msb_u64(2), 2);
        assert_eq!(msb_u64(3), 2);
        assert_eq!(msb_u64(4), 4);
        assert_eq!(msb_u64(7), 4);
        assert_eq!(msb_u64(8), 8);
        assert_eq!(msb_u64(15), 8);
        assert_eq!(msb_u64(16), 16);
        assert_eq!(msb_u64(31), 16);
    }
}
