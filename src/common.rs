mod msb;
mod node;
mod path_iterator;
mod position;
mod storage_map;

pub use msb::MSB;
pub use node::{Node, ParentNode};
pub use path_iterator::IntoPathIterator;
pub use position::Position;
pub use storage_map::{StorageError, StorageMap};

pub const NODE: u8 = 0x01;
pub const LEAF: u8 = 0x00;

pub type Bytes1 = [u8; 1];
pub type Bytes2 = [u8; 2];
pub type Bytes4 = [u8; 4];
pub type Bytes8 = [u8; 8];
pub type Bytes16 = [u8; 16];
pub type Bytes32 = [u8; 32];
