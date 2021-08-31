use crate::common::storage::{ReadError, Storage};

use std::collections::HashMap;
use std::error;

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

impl<Key: 'static, Value> Storage<Key, Value> for StorageMap<Key, Value>
where
    Key: Eq + std::hash::Hash + std::fmt::Debug + std::fmt::Display + Clone,
{
    fn create(&mut self, key: Key, value: Value) -> Result<&Value, Box<dyn error::Error>> {
        self.map.insert(key.clone(), value);
        Ok(self.map.get(&key).unwrap())
    }

    fn get(&self, key: Key) -> Result<&Value, Box<dyn error::Error>> {
        match self.map.get(&key) {
            None => Err(Box::new(ReadError::new(key))),
            Some(record) => Ok(record),
        }
    }

    fn update(&mut self, key: Key, value: Value) -> Result<&Value, Box<dyn error::Error>> {
        match self.map.get_mut(&key) {
            None => Err(Box::new(ReadError::new(key))),
            Some(record) => {
                *record = value;
                Ok(record)
            }
        }
    }

    fn delete(&mut self, key: Key) -> Result<(), Box<dyn error::Error>> {
        match self.map.get(&key) {
            None => Err(Box::new(ReadError::new(key))),
            Some(_) => {
                self.map.remove(&key);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::storage::Storage;
    use crate::common::storage_map::StorageMap;

    #[test]
    fn test_get_returns_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());

        assert_eq!(storage.get(0).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_get_returns_read_error_for_invalid_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());

        assert!(storage.get(1).is_err());
    }

    #[test]
    fn test_update_updates_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());
        let _ = storage.update(0, "Goodbye, World!".to_string());

        assert_eq!(storage.get(0).unwrap(), "Goodbye, World!");
    }

    #[test]
    fn test_update_returns_updated_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());

        assert_eq!(
            storage.update(0, "Goodbye, World!".to_string()).unwrap(),
            "Goodbye, World!"
        );
    }

    #[test]
    fn test_update_returns_read_error_for_invalid_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());

        assert!(storage.update(1, "Goodbye, World!".to_string()).is_err());
    }

    #[test]
    fn test_delete_deletes_the_value_for_the_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        let _ = storage.create(0, "Hello, World!".to_string());
        let _ = storage.delete(0);

        assert!(storage.get(0).is_err());
    }

    #[test]
    fn test_delete_returns_read_error_for_invalid_key() {
        let mut storage = StorageMap::<u32, String>::new();

        assert!(storage.delete(0).is_err());
    }
}
