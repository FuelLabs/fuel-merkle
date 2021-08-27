use crate::storage_binary::storage::{Storage, ReadError};

use std::collections::HashMap;
use std::marker::PhantomData;
use std::error::Error;

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

    fn update(&mut self, key: Key, value: Value) -> Result<&Value, ReadError>{
        let record = self.map.get_mut(&key).ok_or_else(|| "Shit!");
        let r = record.unwrap();
        *r = value;
        Ok(r)
    }

    fn delete(&mut self, key: Key) {
        self.map.remove(&key);
    }
}

#[cfg(test)]
mod test {
    use crate::storage_binary::storage_map::StorageMap;
    use crate::storage_binary::storage::Storage;

    #[test]
    fn test_it() {
        let mut storage = StorageMap::<u32, String>::new();

        storage.create(0, "Hello World!".to_string());

        println!("{:?}", storage);

        let str = storage.get(0);
        println!("{:?}", str);

        storage.update(0, "fuck you".to_string());
        println!("{:?}", storage);

        storage.update(1, "lol".to_string());
    }
}