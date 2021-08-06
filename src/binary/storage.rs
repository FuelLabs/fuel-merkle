use crate::binary::position::Position;

#[derive(Clone)]
pub struct Node {
    key: Position,
    data: [u8; 32],
}

impl Node {
    pub fn new(key: Position, data: [u8; 32]) -> Self {
        Self { key, data }
    }

    pub fn key(&self) -> Position {
        self.key
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

pub trait Storage {
    // CRD interface
    fn create_node(&mut self, key: Position, data: &[u8]);

    fn get_all_nodes(&self) -> Vec<Node>;

    fn read_node(&self, ptr: u64) -> Option<&Node>;

    fn delete_node(&mut self, ptr: u64);
}
