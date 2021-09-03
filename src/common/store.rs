use std::fmt::{Debug};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("generic error occurred")]
    Error(Box<dyn std::error::Error + Send>),
}

pub trait Store<Key, Value> {
    fn insert(&mut self, key: &Key, value: &Value) -> Result<Option<Value>, StoreError>;

    fn remove(&mut self, key: &Key) -> Result<Option<Value>, StoreError>;

    fn get(&self, key: &Key) -> Result<Option<Value>, StoreError>;

    fn contains_key(&self, key: &Key) -> Result<bool, StoreError>;
}
