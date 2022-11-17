use core::{fmt, mem};

pub trait Node {
    type Key;

    fn key_size_in_bits() -> usize {
        mem::size_of::<Self::Key>() * 8
    }

    fn height(&self) -> u32;
    fn leaf_key(&self) -> Self::Key;
    fn is_leaf(&self) -> bool;
    fn is_node(&self) -> bool;
}

pub trait ParentNode: Node
where
    Self: Sized,
{
    type Error: Clone + fmt::Debug;

    fn left_child(&self) -> ChildResult<Self>;
    fn right_child(&self) -> ChildResult<Self>;
}

#[derive(Debug, Clone)]
pub enum ParentNodeError<E: Clone> {
    ChildNotFound,
    Error(E),
}

impl<E: Clone> From<E> for ParentNodeError<E> {
    fn from(e: E) -> Self {
        Self::Error(e)
    }
}

#[allow(type_alias_bounds)]
pub type ChildResult<T: ParentNode> = Result<T, ParentNodeError<T::Error>>;
