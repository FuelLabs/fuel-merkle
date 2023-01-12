use crate::binary::Node;
use crate::common::{Bytes32, Position};

pub type Primitive = (u64, Bytes32);

impl<'a> From<&'a Node> for Primitive {
    fn from(node: &Node) -> Self {
        (node.position().in_order_index(), *node.hash())
    }
}

impl From<Primitive> for Node {
    fn from(buffer: Primitive) -> Self {
        let position = Position::from_in_order_index(buffer.0);
        let hash = buffer.1;
        Node { position, hash }
    }
}

pub trait AsPrimitive {
    fn as_primitive(&self) -> Primitive;
}

impl AsPrimitive for Node {
    fn as_primitive(&self) -> Primitive {
        self.into()
    }
}
