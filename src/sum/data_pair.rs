use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

pub fn join_data_pair(data: &[u8], fee: u64) -> Vec<u8> {
    let mut sum_data = Vec::<u8>::new();
    for d in fee.to_be_bytes().iter() {
        sum_data.push(*d)
    }
    for d in data.iter() {
        sum_data.push(*d)
    }
    sum_data
}

pub fn split_data_pair(data_pair: &[u8]) -> (u64, &[u8]) {
    let fee_data = &data_pair[0..8];
    let mut reader = Cursor::new(fee_data);
    let fee = reader.read_u64::<BigEndian>().unwrap();
    (fee, &data_pair[8..])
}
