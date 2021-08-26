use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::common::position::Position;

#[derive(Clone, Debug)]
pub struct Node<Key> {
    key: Key,
    parent_key: Option<Key>,
    left_key: Option<Key>,
    right_key: Option<Key>,
}

impl<Key> Node<Key>
where
    Key: Clone + std::ops::Add<Output = Key>,
{
    pub fn new(key: Key) -> Self {
        Self {
            key,
            parent_key: None,
            left_key: None,
            right_key: None,
        }
    }

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

    pub fn join(&mut self, other: &mut Self) -> Self {
        let key = self.key() + other.key();
        let mut joined = Self::new(key);

        self.set_parent_key(Some(joined.key()));
        other.set_parent_key(Some(joined.key()));
        joined.set_left_key(Some(self.key()));
        joined.set_right_key(Some(other.key()));

        joined
    }
}

/*pub struct NodeProofIterator<T> {
    current: Option<RefNode<T>>,
    previous: Option<RefNode<T>>,
}

impl<T> NodeProofIterator<T> {
    pub fn new(node: RefNode<T>) -> Self {
        Self {
            current: Some(Rc::clone(&node)),
            previous: Some(Rc::clone(&node)),
        }
    }
}

impl<T> Iterator for NodeProofIterator<T> {
    type Item = RefNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take();
        node.map(|mut n| {
            self.node = n.take_next();
            n
        })
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = RefCell<Node<T>>;
    type IntoIter = NodeProofIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(Some)
    }
}*/

#[cfg(test)]
mod test {
    use super::*;
    type N = Node<u32>;

    #[test]
    fn test() {
        let mut n0 = N::new(0);
        let mut n1 = N::new(1);
        let mut n2 = N::new(2);
        let mut n3 = N::new(3);
        let mut n4 = n0.join(&mut n1);
        let mut n5 = n2.join(&mut n3);
        let mut n6 = n4.join(&mut n5);

        println!("{:?}", n6);
    }
}
