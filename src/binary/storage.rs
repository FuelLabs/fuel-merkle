use bytes::Bytes;

use std::collections::HashMap;

pub struct Node {
    data: Bytes,
    left_child_ptr: Option<Bytes>,
    right_child_ptr: Option<Bytes>,
}

impl Node {
    pub fn new(data: Bytes, left_child_ptr: Option<Bytes>, right_child_ptr: Option<Bytes>) -> Self {
        Self { data, left_child_ptr, right_child_ptr }
    }

    pub fn left_child_ptr(&self) -> &Option<Bytes> {
        &self.left_child_ptr
    }

    pub fn right_child_ptr(&self) -> &Option<Bytes> {
        &self.right_child_ptr
    }
}

pub struct Storage {
    map: HashMap<Bytes, Node>,
}

impl Storage {
    pub fn new() -> Self {
        Self { map: HashMap::<Bytes, Node>::new() }
    }

    pub fn create_leaf(&mut self, data: &[u8]) {
        self.create_node(data, None, None);
    }

    pub fn create_node(&mut self, data: &[u8], left_child_ptr: Option<&[u8]>, right_child_ptr: Option<&[u8]>) {
        let k = Bytes::copy_from_slice(data);
        let node = Node::new(
            k.clone(),
            left_child_ptr.map( |r| Bytes::copy_from_slice(r) ),
            right_child_ptr.map( |r| Bytes::copy_from_slice(r) ),
        );
        self.map.insert(k.clone(), node);
    }

    pub fn read(&self, ptr: Bytes) -> &Node {
        self.map.get(&ptr).unwrap()
    }
}
