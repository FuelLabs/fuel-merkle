use digest::generic_array::GenericArray;
use digest::Digest;
use sha2::Sha256;
use std::fmt;
use std::fmt::Formatter;

use crate::binary::position::Position;

#[derive(Clone)]
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    height: u32,
    position: Position,
    data: T,
}

impl<T> Node<T> {
    pub fn new(next: Option<Box<Node<T>>>, height: u32, position: Position, data: T) -> Self {
        Self {
            next,
            height,
            position,
            data,
        }
    }

    pub fn next(&self) -> &Option<Box<Node<T>>> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<Node<T>>> {
        &mut self.next
    }

    pub fn take_next(&mut self) -> Option<Box<Node<T>>> {
        self.next_mut().take()
    }

    pub fn height(&self) -> u32 {
        self.position.height()
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn next_height(&self) -> Option<u32> {
        self.next().as_ref().map(|next| next.height())
    }

    pub fn next_data(&self) -> Option<&T> {
        self.next().as_ref().map(|next| next.data())
    }
}

type Data = GenericArray<u8, <Sha256 as Digest>::OutputSize>;
impl Node<Data> {
    pub fn to_string(&self) -> String {
        bs58::encode(self.data()).into_string()
    }
}

impl fmt::Debug for Node<Data> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let next_to_string = match self.next() {
            None => String::from("(None)"),
            Some(next) => next.to_string(),
        };
        f.debug_tuple("Node")
            .field(&self.to_string())
            .field(&self.height())
            .field(&next_to_string)
            .finish()
    }
}

impl fmt::Display for Node<Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let next_to_string = match self.next() {
            None => String::from("(None)"),
            Some(ref next) => next.to_string(),
        };
        write!(
            f,
            "Node({}, {}, {})",
            self.to_string(),
            self.height(),
            next_to_string
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_format() {
        let mut data = Data::default();
        for x in &mut data {
            *x = 0xff;
        }
        let node = Node::<Data>::new(None, 0, Position::from_index(0), data);
        let node_str = format!("{}", node);
        assert_eq!(
            "Node(JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG, 0, (None))",
            node_str
        );
    }

    #[test]
    fn test_format_with_next() {
        let mut data = Data::default();
        for x in &mut data {
            *x = 0xff;
        }
        let node_1 = Node::<Data>::new(None, 0, Position::from_index(0), data.clone());
        let node_1_str = format!("{}", node_1);
        let node_2 = Node::<Data>::new(Some(Box::new(node_1)), 0, Position::from_index(0), data.clone());
        let node_2_str = format!("{}", node_2);
        let expected = format!(
            "Node(JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG, 0, {})",
            node_1_str
        );
        assert_eq!(expected, node_2_str);
    }
}
