use crate::storage_binary::storage::{ReadError, Storage};

use std::collections::HashMap;

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

    fn get(&self, key: Key) -> Result<&Value, ReadError<Key>> {
        match self.map.get(&key) {
            None => Err(ReadError::new(key)),
            Some(record) => Ok(record),
        }
    }

    fn update(&mut self, key: Key, value: Value) -> Result<&Value, ReadError<Key>> {
        match self.map.get_mut(&key) {
            None => Err(ReadError::new(key)),
            Some(record) => {
                *record = value;
                Ok(record)
            }
        }
    }

    fn delete(&mut self, key: Key) {
        self.map.remove(&key);
    }
}

#[cfg(test)]
mod test {
    use crate::storage_binary::storage::{ReadError, Storage};
    use crate::storage_binary::storage_map::StorageMap;

    #[test]
    fn test_get_returns_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        storage.create(0, "Hello, World!".to_string());

        assert_eq!(storage.get(0).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_get_returns_read_error_for_invalid_key() {
        let mut storage = StorageMap::<u32, String>::new();
        storage.create(0, "Hello, World!".to_string());

        assert_eq!(storage.get(1).unwrap_err(), ReadError::new(1));
    }

    #[test]
    fn test_update_updates_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        storage.create(0, "Hello, World!".to_string());
        let _ = storage.update(0, "Goodbye, World!".to_string());

        assert_eq!(storage.get(0).unwrap(), "Goodbye, World!");
    }

    #[test]
    fn test_update_returns_updated_value_for_given_key() {
        let mut storage = StorageMap::<u32, String>::new();
        storage.create(0, "Hello, World!".to_string());

        assert_eq!(
            storage.update(0, "Goodbye, World!".to_string()).unwrap(),
            "Goodbye, World!"
        );
    }

    #[test]
    fn test_update_returns_read_error_for_invalid_key() {
        let mut storage = StorageMap::<u32, String>::new();
        storage.create(0, "Hello, World!".to_string());

        assert_eq!(
            storage
                .update(1, "Goodbye, World!".to_string())
                .unwrap_err(),
            ReadError::new(1)
        );
    }
}
