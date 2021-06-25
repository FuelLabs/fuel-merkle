pub struct Node<Data> {
    data: Data,
    left_child_ptr: Option<Data>,
    right_child_ptr: Option<Data>,
}

impl<Data> Node<Data> {
    pub fn new(data: Data, left_child_ptr: Option<Data>, right_child_ptr: Option<Data>) -> Self {
        Self {
            data,
            left_child_ptr,
            right_child_ptr,
        }
    }

    pub fn left_child_ptr(&self) -> &Option<Data> {
        &self.left_child_ptr
    }

    pub fn right_child_ptr(&self) -> &Option<Data> {
        &self.right_child_ptr
    }
}

pub trait Storage {
    type Key;

    // CRUD interface

    fn create_node(
        &mut self,
        data: &[u8],
        left_child_ptr: Option<&[u8]>,
        right_child_ptr: Option<&[u8]>,
    ) -> &Node<Key>;

    fn read_node(&self, ptr: Key) -> Option<&Node<Key>>;

    fn delete_node(&mut self, ptr: Key);

    //

    fn create_leaf(&mut self, data: &[u8]) ->&Node<Key> {
        self.create_node(data, None, None)
    }
}
