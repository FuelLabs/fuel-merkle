use core::mem;

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
    type Error;

    fn left_child(&self) -> ChildResult<Self>;
    fn right_child(&self) -> ChildResult<Self>;
}

#[derive(Debug, Clone)]
pub enum ChildError<Key, E> {
    ChildNotFound(Key),
    Error(E),
}

impl<Key, E> From<E> for ChildError<Key, E> {
    fn from(e: E) -> Self {
        Self::Error(e)
    }
}

#[allow(type_alias_bounds)]
pub type ChildResult<T: ParentNode> = Result<T, ChildError<T::Key, T::Error>>;
