use crate::binary::{BinaryNode, Node};
use crate::common::{Bytes32, Position};

pub type Primitive = (u64, Bytes32);

impl BinaryNode for Primitive {
    fn position(&self) -> Position {
        Position::from_in_order_index(self.0)
    }

    fn hash(&self) -> &Bytes32 {
        &self.1
    }
}

impl From<&Node> for Primitive {
    fn from(node: &Node) -> Self {
        (node.position().in_order_index(), *node.hash())
    }
}

impl From<Primitive> for Node {
    fn from(primitive: Primitive) -> Self {
        let position = primitive.position();
        let hash = *primitive.hash();
        Node { position, hash }
    }
}
