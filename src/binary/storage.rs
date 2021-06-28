use bytes::Bytes;
use digest::generic_array::GenericArray;
use digest::Digest;
use sha2::Sha256;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
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

type Data = Bytes;
impl Node<Data> {
    pub fn to_string(&self) -> String {
        bs58::encode(&self.data).into_string()
    }
}

impl fmt::Debug for Node<Data> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_to_string = match self.left_child_ptr() {
            None => String::from("(None)"),
            Some(ref left) => bs58::encode(left).into_string(),
        };
        let right_to_string = match self.right_child_ptr() {
            None => String::from("(None)"),
            Some(ref right) => bs58::encode(right).into_string(),
        };
        f.debug_tuple("Node")
            .field(&self.to_string())
            .field(&left_to_string)
            .field(&right_to_string)
            .finish()
    }
}

impl fmt::Display for Node<Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let left_to_string = match self.left_child_ptr() {
            None => String::from("(None)"),
            Some(ref left) => bs58::encode(left).into_string(),
        };
        let right_to_string = match self.right_child_ptr() {
            None => String::from("(None)"),
            Some(ref right) => bs58::encode(right).into_string(),
        };
        write!(
            f,
            "Node({}, {}, {})",
            self.to_string(),
            left_to_string,
            right_to_string
        )
    }
}

pub trait Storage {
    type Key;

    fn new() -> Self;

    // CRD interface

    fn create_node(
        &mut self,
        data: &[u8],
        left_child_ptr: Option<&[u8]>,
        right_child_ptr: Option<&[u8]>,
    );

    fn read_node(&self, ptr: &Self::Key) -> Option<&Node<Self::Key>>;

    fn delete_node(&mut self, ptr: &Self::Key);

    //

    fn create_leaf(&mut self, data: &[u8]) {
        self.create_node(data, None, None);
    }
}
