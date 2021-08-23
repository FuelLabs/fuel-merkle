use crate::common::position::Position;

#[derive(Clone)]
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    position: Position,
    data: T,
}

impl<T> Node<T> {
    pub fn new(next: Option<Box<Node<T>>>, position: Position, data: T) -> Self {
        Self {
            next,
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

pub struct NodeIterator<T> {
    node: Option<Box<Node<T>>>,
}

impl<T> Iterator for NodeIterator<T> {
    type Item = Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.take();
        node.map(|mut n| {
            self.node = n.take_next();
            n
        })
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = Box<Node<T>>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            node: Some(Box::new(self)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_iter() {
        type N = Node<u32>;

        let n0 = Box::new(N::new(None, Position::from_in_order_index(0), 3));
        let n1 = Box::new(N::new(Some(n0), Position::from_in_order_index(1), 2));
        let n2 = Box::new(N::new(Some(n1), Position::from_in_order_index(2), 1));
        let n3 = Box::new(N::new(Some(n2), Position::from_in_order_index(3), 0));

        for (i, n) in n3.into_iter().enumerate() {
            assert_eq!(i as u32, *n.data());
        }
    }
}
