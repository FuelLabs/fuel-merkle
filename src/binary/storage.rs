use bytes::Bytes;
use std::fmt;
use std::fmt::Formatter;

use crate::binary::position::Position;

use generic_array;

#[derive(Clone)]
pub struct Node {
    key: Position,
    data: [u8; 32],
}

impl Node {
    pub fn new(key: u64, data: [u8; 32]) -> Self {
        Self {
            key: Position::from_index(key),
            data,
        }
    }

    pub fn key(&self) -> u64 {
        self.key.value()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/*type Data = Bytes;
impl Node<Data> {
    pub fn to_string(&self) -> String {
        bs58::encode(&self.data).into_string()
    }
}

impl fmt::Display for Node<Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Node({}, {})", self.key.value() as i64, self.to_string(),)
    }
}*/

pub trait Storage {
    fn new() -> Self;

    // CRD interface
    fn create_node(&mut self, key: u64, data: &[u8]);

    fn get_all_nodes(&self) -> Vec<Node>;

    fn read_node(&self, ptr: u64) -> Option<&Node>;

    fn delete_node(&mut self, ptr: u64);
}
