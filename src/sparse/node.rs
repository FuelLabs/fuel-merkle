use std::cell::Cell;
use fuel_storage::Storage;

use crate::common::{ParentNode, Position};

#[derive(Clone)]
pub struct Node<Key> {
    position: Position,
    key: Key,
}

impl<Key> Node<Key>
    where
        Key: Clone,
{
    pub fn new(
        position: Position,
        key: Key,
    ) -> Self {
        let n = Self {
            position,
            key,
        };
        n
    }

    pub fn key(&self) -> Key {
        self.key.clone()
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use fuel_storage::Storage;
    use crate::common::{IntoPathIterator, Position, StorageError, StorageMap};
    use crate::sparse::Node;

    #[test]
    fn test_it() {
        let mut s = StorageMap::<Position, Node::<u32>>::new();

        //       03
        //      /  \
        //     /    \
        //   01      05
        //  /  \    /  \
        // 00  02  04  06
        // 00  01  02  03

        let p0 = Position::from_in_order_index(0);
        let n0 = Node::<u32>::new(p0, 0);
        s.insert(&p0, &n0);

        let p1 = Position::from_in_order_index(1);
        let n1 = Node::<u32>::new(p1, 1);
        s.insert(&p1, &n1);

        let p2 = Position::from_in_order_index(2);
        let n2 = Node::<u32>::new(p2, 2);
        s.insert(&p2, &n2);

        let p3 = Position::from_in_order_index(3);
        let n3 = Node::<u32>::new(p3, 3);
        s.insert(&p3, &n3);

        let p4 = Position::from_in_order_index(4);
        let n4 = Node::<u32>::new(p4, 4);
        s.insert(&p4, &n4);

        let p5 = Position::from_in_order_index(5);
        let n5 = Node::<u32>::new(p5, 5);
        s.insert(&p5, &n5);

        let p6 = Position::from_in_order_index(6);
        let n6 = Node::<u32>::new(p6, 6);
        s.insert(&p6, &n6);

        let leaf = Position::from_leaf_index(1);
        let root = s.get(&Position::from_in_order_index(3)).unwrap().unwrap();
        let iter = leaf.into_path_iter(&root.as_ref().position);

        let mut hashes = Vec::<u32>::new();
        for p in iter {
            let h = s.get(&p).unwrap().unwrap();
            hashes.push(h.key())
        }
        println!("{:?}", hashes);
    }
}