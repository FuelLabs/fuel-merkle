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
    pub fn new(key: Position, data: [u8; 32]) -> Self {
        Self {
            key,
            data,
        }
    }

    pub fn key(&self) -> Position {
        self.key
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
    // CRD interface
    fn create_node(&mut self, key: Position, data: &[u8]);

    fn get_all_nodes(&self) -> Vec<Node>;

    fn read_node(&self, ptr: u64) -> Option<&Node>;

    fn delete_node(&mut self, ptr: u64);
}
