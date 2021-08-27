use std::fmt;
use std::fmt::{Debug, Formatter};

// use crate::storage_binary::storage::Storage;
use crate::storage_binary::storage_map::StorageMap;
use crate::common::position::Position;

#[derive(Clone)]
pub struct Node<Key> {
    position: Position,
    key: Key,
    parent_key: Option<Key>,
    left_key: Option<Key>,
    right_key: Option<Key>,
}

impl<Key> Debug for Node<Key>
where
    Key: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("position", &self.position)
            .field("key", &self.key)
            .field("parent_key", &self.parent_key)
            .field("left_key", &self.left_key)
            .field("right_key", &self.right_key)
            .finish()
    }
}

impl<Key> Node<Key>
where
    Key: Clone
{
    pub fn new(position: Position, key: Key) -> Self {
        Self {
            position,
            key,
            parent_key: None,
            left_key: None,
            right_key: None,
        }
    }

    pub fn position(&self) -> Position { self.position }

    pub fn key(&self) -> Key {
        self.key.clone()
    }

    pub fn parent_key(&self) -> Option<Key> {
        self.parent_key.clone()
    }

    pub fn left_key(&self) -> Option<Key> {
        self.left_key.clone()
    }

    pub fn right_key(&self) -> Option<Key> {
        self.right_key.clone()
    }

    pub fn set_parent_key(&mut self, key: Option<Key>) {
        self.parent_key = key.clone();
    }

    pub fn set_left_key(&mut self, key: Option<Key>) {
        self.left_key = key.clone();
    }

    pub fn set_right_key(&mut self, key: Option<Key>) {
        self.right_key = key.clone();
    }
}

