#[derive(Clone)]
pub struct TestNode<T> {
    next: Option<Box<TestNode<T>>>,
    height: u32,
    data: T,
}

impl<T> TestNode<T> {
    pub fn new(next: Option<Box<TestNode<T>>>, height: u32, data: T) -> Self {
        Self { next, height, data }
    }

    pub fn next(&self) -> &Option<Box<TestNode<T>>> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<TestNode<T>>> {
        &mut self.next
    }

    pub fn take_next(&mut self) -> Option<Box<TestNode<T>>> {
        self.next_mut().take()
    }
}

impl<T> IntoIterator for TestNode<T> {
    type Item = Box<TestNode<T>>;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            cursor: Some(Box::new(self)),
        }
    }
}

pub struct NodeIterator<T> {
    cursor: Option<Box<TestNode<T>>>,
}

impl<T> NodeIterator<T> {
    fn new(node: Option<Box<TestNode<T>>>) -> Self {
        Self { cursor: node }
    }
}

impl<T> Iterator for NodeIterator<T> {
    type Item = Box<TestNode<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.cursor.take();
        if let Some(ref mut cursor) = current {
            self.cursor = cursor.take_next();
        }
        current
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use digest::generic_array::GenericArray;
    use digest::Digest;
    use sha2::Sha256;

    type Data = GenericArray<u8, <Sha256 as Digest>::OutputSize>;

    #[test]
    fn test_iterate() {
        let mut data = Data::default();
        for x in &mut data {
            *x = 0xff;
        }

        let node_1 = TestNode::<Data>::new(None, 0, data.clone());
        let node_2 = TestNode::<Data>::new(Some(Box::new(node_1)), 1, data.clone());
        let node_3 = TestNode::<Data>::new(Some(Box::new(node_2)), 2, data.clone());

        let expected_v = vec![2u32, 1, 0];
        let mut v = Vec::<u32>::new();
        for n in node_3.into_iter() {
            v.push(n.height);
        }

        assert_eq!(v, expected_v);
    }
}
