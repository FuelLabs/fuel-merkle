use crate::binary::position::Position;
use crate::binary::storage::{Node, Storage};

use std::collections::HashMap;
use std::convert::TryInto;

pub struct StorageMap {
    map: HashMap<u64, Node>,
}

impl StorageMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::<u64, Node>::new(),
        }
    }

    fn insert_node(&mut self, key: Position, node: Node) {
        self.map.insert(key.index(), node);
    }
}

impl Storage for StorageMap {
    fn create_node(&mut self, key: Position, data: &[u8]) {
        let node = Node::new(key, data.try_into().unwrap());
        self.insert_node(key, node.clone());
    }

    fn get_all_nodes(&self) -> Vec<Node> {
        self.map.values().cloned().collect()
    }

    fn read_node(&self, key: Position) -> Option<&Node> {
        self.map.get(&key.index())
    }

    fn delete_node(&mut self, ptr: u64) {
        self.map.remove(&ptr);
    }
}
