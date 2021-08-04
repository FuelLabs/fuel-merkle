use crate::binary::storage::{Node, Storage};

use bytes::Bytes;
use std::collections::HashMap;
use std::io::Read;
use std::convert::TryInto;

pub struct StorageMap {
    map: HashMap<u64, Node>,
}

impl StorageMap {
    fn insert_node(&mut self, key: u64, node: Node) {
        self.map.insert(key, node);
    }
}

impl Storage for StorageMap {
    fn new() -> Self {
        Self {
            map: HashMap::<u64, Node>::new(),
        }
    }

    fn create_node(&mut self, key: u64, data: &[u8]) {
        let node = Node::new(key, data.try_into().unwrap());
        // println!("{}", &node);
        self.insert_node(key, node.clone());
    }

    fn get_all_nodes(&self) -> Vec<Node> {
        self.map.values().cloned().collect()
    }

    fn read_node(&self, ptr: u64) -> Option<&Node> {
        self.map.get(&ptr)
    }

    fn delete_node(&mut self, ptr: u64) {
        self.map.remove(&ptr);
    }
}
