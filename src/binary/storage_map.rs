use crate::binary::storage::{Node, Storage};

use bytes::Bytes;
use std::collections::HashMap;

pub struct StorageMap {
    map: HashMap<Bytes, Node<Bytes>>,
}

impl StorageMap {
    fn insert_node(&mut self, key: Bytes, node: Node<Bytes>) {
        self.map.insert(key.clone(), node);
    }
}

impl Storage for StorageMap {
    type Key = Bytes;

    fn new() -> Self {
        Self {
            map: HashMap::<Bytes, Node<Bytes>>::new(),
        }
    }

    fn create_node(
        &mut self,
        data: &[u8],
        left_child_ptr: Option<&[u8]>,
        right_child_ptr: Option<&[u8]>,
    ) {
        let key = Bytes::copy_from_slice(data);
        let node = Node::<Self::Key>::new(
            key.clone(),
            left_child_ptr.map(|r| Bytes::copy_from_slice(r)),
            right_child_ptr.map(|r| Bytes::copy_from_slice(r)),
        );
        println!("{:?}", &node);
        self.insert_node(key, node.clone());
    }

    fn read_node(&self, ptr: &Self::Key) -> Option<&Node<Self::Key>> {
        self.map.get(ptr)
    }

    fn delete_node(&mut self, ptr: &Self::Key) {
        self.map.remove(ptr);
    }
}
