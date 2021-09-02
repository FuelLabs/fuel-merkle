use fuel_vm::data::{DataError, Key, Storage, Value};

use std::collections::HashMap;
use std::hash::Hash;

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

impl<K, V> Storage<K, V> for StorageMap<K, V>
where
    K: Key + Copy + Hash + Eq,
    V: Value + Clone,
{
    fn insert(&mut self, key: &K, value: &V) -> Result<Option<V>, DataError> {
        self.map.insert(*key, value.clone());
        Ok(Some(value.clone()))
    }

    fn remove(&mut self, key: &K) -> Result<Option<V>, DataError> {
        let value = self.map.remove(key);
        Ok(value)
    }

    fn get(&self, key: &K) -> Result<Option<V>, DataError> {
        let result = self.map.get(key);
        Ok(result.cloned())
    }

    fn contains_key(&self, key: &K) -> Result<bool, DataError> {
        Ok(self.map.contains_key(key))
    }
}

/*
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
}*/

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    struct MyKey(u32);

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct MyValue(u32);

    impl Key for MyKey {}
    impl Value for MyValue {}

    #[test]
    fn test_get_returns_value_for_given_key() {
        let key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));

        assert_eq!(storage.get(&key).unwrap(), Some(MyValue(0)));
    }
    #[test]
    fn test_get_returns_none_for_invalid_key() {
        let key = MyKey(0);
        let invalid_key = MyKey(1);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));

        assert_eq!(storage.get(&invalid_key).unwrap(), None);
    }

    #[test]
    fn test_insert_existing_key_updates_value_for_given_key() {
        let key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));
        let _ = storage.insert(&key, &MyValue(1));

        assert_eq!(storage.get(&key).unwrap(), Some(MyValue(1)));
    }

    #[test]
    fn test_remove_deletes_the_value_for_given_key() {
        let key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));
        let _ = storage.remove(&key);

        assert_eq!(storage.get(&key).unwrap(), None);
    }

    #[test]
    fn test_remove_returns_the_deleted_value_for_given_key() {
        let key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));

        assert_eq!(storage.remove(&key).unwrap(), Some(MyValue(0)));
    }

    #[test]
    fn test_remove_returns_none_for_invalid_key() {
        let invalid_key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();

        assert_eq!(storage.remove(&invalid_key).unwrap(), None);
    }

    #[test]
    fn test_contains_key_returns_true_for_valid_key() {
        let key = MyKey(0);
        let mut storage = StorageMap::<MyKey, MyValue>::new();
        let _ = storage.insert(&key, &MyValue(0));

        assert_eq!(storage.contains_key(&key).unwrap(), true);
    }

    #[test]
    fn test_contains_key_returns_false_for_invalid_key() {
        let invalid_key = MyKey(0);
        let storage = StorageMap::<MyKey, MyValue>::new();

        assert_eq!(storage.contains_key(&invalid_key).unwrap(), false);
    }
}
