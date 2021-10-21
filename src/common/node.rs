pub trait Node {
    type Key;

    fn key(&self) -> Self::Key;
    fn is_leaf(&self) -> bool;
}

pub trait ParentNode: Node {
    fn left_child(&self) -> Self;
    fn right_child(&self) -> Self;
}
