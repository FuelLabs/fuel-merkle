use std::convert::TryFrom;

pub fn join_data_pair(fee: u64, data: &[u8]) -> [u8; 40] {
    assert_eq!(data.len(), 32);

    let mut sum_data = [0u8; 40];
    sum_data[0..8].copy_from_slice(&fee.to_be_bytes());
    sum_data[8..].copy_from_slice(data);
    sum_data
}

pub fn split_data_pair(data_pair: &[u8]) -> (u64, &[u8]) {
    assert_eq!(data_pair.len(), 40);

    let (l, r) = data_pair.split_at(8);
    let fee = <[u8; 8]>::try_from(l).unwrap();
    let fee = u64::from_be_bytes(fee);
    (fee, r)
}
