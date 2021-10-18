mod node;
mod path_iterator;
mod position;
mod storage_map;

pub use node::{Node, ParentNode};
pub use position::Position;
pub use storage_map::{StorageError, StorageMap};
pub use path_iterator::IntoPathIterator;

pub fn get_bit_at_index_from_msb_u64(key: u64, index: u32) -> u8 {
    let shift = 1 << index;
    (key & shift != 0) as u8
}
