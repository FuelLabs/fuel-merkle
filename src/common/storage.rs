use std::error;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub struct ReadError<Key> {
    key: Key,
}

impl<Key> ReadError<Key> {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

impl<Key: Display> Display for ReadError<Key> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to read {}", self.key)
    }
}

impl<Key: Debug> Debug for ReadError<Key> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadError").field("Key", &self.key).finish()
    }
}

impl<Key: Debug + Display> Error for ReadError<Key> {}

pub trait Storage<Key, Value> {
    fn create(&mut self, key: Key, value: Value) -> Result<&Value, Box<dyn error::Error>>;

    fn get(&self, key: Key) -> Result<&Value, Box<dyn error::Error>>;

    fn update(&mut self, key: Key, value: Value) -> Result<&Value, Box<dyn error::Error>>;

    fn delete(&mut self, key: Key) -> Result<(), Box<dyn error::Error>>;
}
