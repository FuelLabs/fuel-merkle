use crate::binary::storage::{Storage, Node};

use std::collections::HashMap;
use bytes::Bytes;

pub struct StorageMap {
    map: HashMap<Key, Node<Bytes>>,
}

impl StorageMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Key, Node<Bytes>>::new(),
        }
    }

    fn insert_node(&mut self, key: Key, node: Node<Bytes>) {
        self.map.insert(key.clone(), node);
    }
}

impl Storage for StorageMap {
    type Key = Bytes;

    fn create_node(
        &mut self,
        data: &[u8],
        left_child_ptr: Option<&[u8]>,
        right_child_ptr: Option<&[u8]>,
    ) {
        let key = Bytes::copy_from_slice(data);
        let node = Node::<Key>::new(
            key.clone(),
            left_child_ptr.map(|r| Bytes::copy_from_slice(r)),
            right_child_ptr.map(|r| Bytes::copy_from_slice(r)),
        );
        self.insert_node(key, node);
    }

    fn read_node(&self, ptr: Key) -> &Node<Key> {
        self.map.get(&ptr).unwrap()
    }

    fn delete_node(&mut self, ptr: Key) {
        self.map.remove(ptr);
    }
}

#[cfg(test)]
mod test {
    use crate::binary::storage_map::StorageMap;
    use crate::binary::storage::Storage;

    #[test]
    fn test_it() {
        let mut s = StorageMap::new();
        s.create_leaf("Hello World".as_bytes())
    }
}