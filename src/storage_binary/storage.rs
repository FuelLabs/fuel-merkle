use std::error::Error;
use std::fmt::{Debug, Formatter, Display};

pub struct ReadError;

impl Debug for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadError")
            .field("Error", &"Something wrong!".to_string())
            .finish()
    }
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shit son!!")
    }
}

impl Error for ReadError {}

pub trait Storage<Key, Value> {
    // CRD interface
    fn create(&mut self, key: Key, value: Value);

    fn get(&self, key: Key) -> Option<&Value>;

    fn update(&mut self, key: Key, value: Value) -> Result<&Value, ReadError>;

    fn delete(&mut self, key: Key);
}
