use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::common::position::Position;
use crate::storage_binary::storage::Storage;

#[derive(Clone, PartialEq)]
pub struct Node<Key> {
    position: Position,
    key: Key,
    parent_key: Option<Key>,
    left_key: Option<Key>,
    right_key: Option<Key>,
}

impl<Key> Debug for Node<Key>
where
    Key: Debug,
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
    Key: Clone,
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

    pub fn position(&self) -> Position {
        self.position
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

    pub fn iter<'storage>(
        &mut self,
        storage: &'storage mut dyn Storage<Key, Self>,
    ) -> ProofIter<'storage, Key> {
        ProofIter::new(storage, self)
    }
}

pub struct ProofIter<'storage, Key> {
    storage: &'storage mut dyn Storage<Key, Node<Key>>,
    prev: Option<Node<Key>>,
    curr: Option<Node<Key>>,
}

impl<'storage, Key> ProofIter<'storage, Key>
where
    Key: Clone,
{
    pub fn new(storage: &'storage mut dyn Storage<Key, Node<Key>>, node: &Node<Key>) -> Self {
        let curr = storage
            .get(node.parent_key().unwrap())
            .ok()
            .unwrap()
            .clone();
        Self {
            storage,
            prev: Some(node.clone()),
            curr: Some(curr),
        }
    }
}

impl<'storage, Key> Iterator for ProofIter<'storage, Key>
where
    Key: Clone + std::cmp::PartialEq,
{
    type Item = Node<Key>;

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.prev.take();
        let mut current = self.curr.take();

        let node = current.as_ref().map(|curr| {
            let prev = previous.unwrap();
            if curr.left_key().unwrap() == prev.key() {
                self.storage
                    .get(curr.right_key().unwrap())
                    .ok()
                    .unwrap()
                    .clone()
            } else {
                self.storage
                    .get(curr.left_key().unwrap())
                    .ok()
                    .unwrap()
                    .clone()
            }
        });

        self.curr = current
            .as_ref()?
            .parent_key()
            .map(|key| self.storage.get(key).ok().unwrap().clone());
        self.prev = current.take();

        node
    }
}

#[cfg(test)]
mod test {
    use crate::common::position::Position;
    use crate::storage_binary::node::Node;
    use crate::storage_binary::storage::Storage;
    use crate::storage_binary::storage_map::StorageMap;

    #[test]
    pub fn test_proof_iter() {
        type N = Node<u32>;
        let mut storage_map = StorageMap::<u32, N>::new();

        //               07
        //              /  \
        //             /    \
        //            /      \
        //           /        \
        //          /          \
        //         /            \
        //       03              11
        //      /  \            /  \
        //     /    \          /    \
        //   01      05       09     \
        //  /  \    /  \     /  \     \
        // 00  02  04  06   08  10    12
        // 00  01  02  03   04  05    06

        let mut leaf_0 = N::new(Position::from_leaf_index(0), 0);
        let mut leaf_1 = N::new(Position::from_leaf_index(1), 2);
        let mut leaf_2 = N::new(Position::from_leaf_index(2), 4);
        let mut leaf_3 = N::new(Position::from_leaf_index(3), 6);
        let mut leaf_4 = N::new(Position::from_leaf_index(4), 8);
        let mut leaf_5 = N::new(Position::from_leaf_index(5), 10);
        let mut leaf_6 = N::new(Position::from_leaf_index(6), 12);

        let mut node_1 = N::new(Position::from_in_order_index(1), 1);
        leaf_0.set_parent_key(Some(node_1.key()));
        leaf_1.set_parent_key(Some(node_1.key()));
        node_1.set_left_key(Some(leaf_0.key()));
        node_1.set_right_key(Some(leaf_1.key()));

        let mut node_5 = N::new(Position::from_in_order_index(5), 5);
        leaf_2.set_parent_key(Some(node_5.key()));
        leaf_3.set_parent_key(Some(node_5.key()));
        node_5.set_left_key(Some(leaf_2.key()));
        node_5.set_right_key(Some(leaf_3.key()));

        let mut node_9 = N::new(Position::from_in_order_index(9), 9);
        leaf_4.set_parent_key(Some(node_9.key()));
        leaf_5.set_parent_key(Some(node_9.key()));
        node_9.set_left_key(Some(leaf_4.key()));
        node_9.set_right_key(Some(leaf_5.key()));

        let mut node_3 = N::new(Position::from_in_order_index(3), 3);
        node_1.set_parent_key(Some(node_3.key()));
        node_5.set_parent_key(Some(node_3.key()));
        node_3.set_left_key(Some(node_1.key()));
        node_3.set_right_key(Some(node_5.key()));

        let mut node_11 = N::new(Position::from_in_order_index(11), 11);
        node_9.set_parent_key(Some(node_11.key()));
        leaf_6.set_parent_key(Some(node_11.key()));
        node_11.set_left_key(Some(node_9.key()));
        node_11.set_right_key(Some(leaf_6.key()));

        let mut node_7 = N::new(Position::from_in_order_index(7), 7);
        node_3.set_parent_key(Some(node_7.key()));
        node_11.set_parent_key(Some(node_7.key()));
        node_7.set_left_key(Some(node_3.key()));
        node_7.set_right_key(Some(node_11.key()));

        storage_map.create(leaf_0.key(), leaf_0.clone());
        storage_map.create(leaf_1.key(), leaf_1.clone());
        storage_map.create(leaf_2.key(), leaf_2.clone());
        storage_map.create(leaf_3.key(), leaf_3.clone());
        storage_map.create(leaf_4.key(), leaf_4.clone());
        storage_map.create(leaf_5.key(), leaf_5.clone());
        storage_map.create(leaf_6.key(), leaf_6.clone());
        storage_map.create(node_1.key(), node_1.clone());
        storage_map.create(node_5.key(), node_5.clone());
        storage_map.create(node_9.key(), node_9.clone());
        storage_map.create(node_3.key(), node_3.clone());
        storage_map.create(node_11.key(), node_11.clone());
        storage_map.create(node_7.key(), node_7.clone());

        let iter = leaf_0.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_1.clone(), node_5.clone(), node_11.clone()));

        let iter = leaf_1.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_0.clone(), node_5.clone(), node_11.clone()));

        let iter = leaf_2.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_3.clone(), node_1.clone(), node_11.clone()));

        let iter = leaf_3.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_2.clone(), node_1.clone(), node_11.clone()));

        let iter = leaf_4.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_5.clone(), leaf_6.clone(), node_3.clone()));

        let iter = leaf_5.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(leaf_4.clone(), leaf_6.clone(), node_3.clone()));

        let iter = leaf_6.iter(&mut storage_map);
        let col: Vec<N> = iter.collect();
        assert_eq!(col, vec!(node_9.clone(), node_3.clone()));
    }
}
