pub trait Node {
    fn index(&self) -> u64;
    fn height(&self) -> u32;
}

pub trait ParentNode : Node {
    fn is_ancestor_of(&self, descendent: &Self) -> bool;
    fn left_child(&self) -> Self;
    fn right_child(&self) -> Self;
}
