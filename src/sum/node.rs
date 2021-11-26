#[derive(Clone)]
pub struct Node<Key> {
    height: u32,
    key: Key,
    fee: u32,
    left_key: Option<Key>,
    right_key: Option<Key>,
}

impl<Key> Node<Key>
where
    Key: Clone,
{
    pub fn new(height: u32, key: Key, fee: u32) -> Self {
        Self {
            height,
            key,
            fee,
            left_key: None,
            right_key: None,
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn key(&self) -> Key {
        self.key.clone()
    }

    pub fn fee(&self) -> u32 {
        self.fee
    }

    pub fn left_key(&self) -> Option<Key> {
        self.left_key.clone()
    }

    pub fn right_key(&self) -> Option<Key> {
        self.right_key.clone()
    }

    pub fn set_left_key(&mut self, key: Option<Key>) {
        self.left_key = key;
    }

    pub fn set_right_key(&mut self, key: Option<Key>) {
        self.right_key = key;
    }
}
