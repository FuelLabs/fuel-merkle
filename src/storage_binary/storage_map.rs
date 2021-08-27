use crate::storage_binary::storage::Storage;

use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct StorageMap<Key, Value> {
    map: HashMap<Key, Value>,
}

impl<Key, Value> StorageMap<Key, Value> {
    pub fn new() -> Self {
        Self {
            map: HashMap::<Key, Value>::new(),
        }
    }
}

impl<Key, Value> Storage<Key, Value> for StorageMap<Key, Value>
where
    Key: Eq + std::hash::Hash,
{
    fn create(&mut self, key: Key, value: Value) {
        self.map.insert(key, value);
    }

    fn get(&self, key: Key) -> Option<&Value> {
        self.map.get(&key)
    }

    fn delete(&mut self, key: Key) {
        self.map.remove(&key);
    }
}
