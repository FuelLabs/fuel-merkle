use crate::binary::storage::{Node, Storage};

use bytes::Bytes;
use std::collections::HashMap;

pub struct StorageMap {
    map: HashMap<u64, Node<Bytes>>,
}

impl StorageMap {
    fn insert_node(&mut self, key: u64, node: Node<Bytes>) {
        self.map.insert(key, node);
    }
}

impl Storage for StorageMap {
    fn new() -> Self {
        Self {
            map: HashMap::<u64, Node<Bytes>>::new(),
        }
    }

    fn create_node(&mut self, key: u64, data: &[u8]) {
        let node = Node::<Bytes>::new(key, Bytes::copy_from_slice(data));
        println!("{}", &node);
        self.insert_node(key, node.clone());
    }

    fn read_node(&self, ptr: u64) -> Option<&Node<Bytes>> {
        self.map.get(&ptr)
    }

    fn delete_node(&mut self, ptr: u64) {
        self.map.remove(&ptr);
    }
}
