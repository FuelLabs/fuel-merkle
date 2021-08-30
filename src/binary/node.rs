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
