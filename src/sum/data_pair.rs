use std::convert::TryFrom;

pub fn join_data_pair(fee: u64, data: &[u8]) -> Vec<u8> {
    let mut sum_data = Vec::<u8>::with_capacity(std::mem::size_of::<u64>() + data.len());
    sum_data.extend_from_slice(&fee.to_be_bytes());
    sum_data.extend_from_slice(data);
    sum_data
}

pub fn split_data_pair(data_pair: &[u8]) -> (u64, &[u8]) {
    assert!(data_pair.len() > 8);

    let (l, r) = data_pair.split_at(8);
    let fee = <[u8; 8]>::try_from(l).unwrap();
    let fee = u64::from_be_bytes(fee);

    (fee, r)
}
