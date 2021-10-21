use crate::common::Bytes32;

pub trait Node {
    fn key(&self) -> Bytes32;
    // fn height(&self) -> u32;
    fn is_leaf(&self) -> bool;
    // fn sibling(&self) -> Self;
}

pub trait ParentNode: Node {
    // fn is_ancestor_of(&self, descendent: &Self) -> bool;
    fn left_child(&self) -> Self;
    fn right_child(&self) -> Self;
}
