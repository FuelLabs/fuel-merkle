#[derive(Debug, Clone)]
pub struct Subtree<T> {
    node: T,
    next: Option<Box<Subtree<T>>>,
}

impl<T> Subtree<T> {
    pub fn new(node: T, next: Option<Box<Subtree<T>>>) -> Self {
        Self { node, next }
    }

    pub fn next(&self) -> &Option<Box<Subtree<T>>> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut Option<Box<Subtree<T>>> {
        &mut self.next
    }

    pub fn take_next(&mut self) -> Option<Box<Subtree<T>>> {
        self.next_mut().take()
    }

    pub fn node(&self) -> &T {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut T {
        &mut self.node
    }

    pub fn next_node(&self) -> Option<&T> {
        self.next().as_ref().map(|next| next.node())
    }

    pub fn next_node_mut(&mut self) -> Option<&mut T> {
        self.next_mut().as_mut().map(|next| next.node_mut())
    }

    pub fn into_iter(self) -> SubtreeIter<T> {
        SubtreeIter::<T> {
            current: Some(Box::new(self)),
        }
    }
}

pub struct SubtreeIter<T> {
    current: Option<Box<Subtree<T>>>,
}

impl<T> Iterator for SubtreeIter<T> {
    type Item = Box<Subtree<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut subtree = self.current.take();
        if let Some(s) = subtree.as_mut() {
            self.current = s.take_next()
        }
        subtree
    }
}

#[cfg(test)]
mod test {
    use crate::storage_binary::subtree::Subtree;

    #[test]
    pub fn test_subtree_iter() {
        type SubtreeT = Subtree<u32>;

        let h0 = Box::new(SubtreeT::new(0, None));
        let h1 = Box::new(SubtreeT::new(1, Some(h0)));
        let h2 = Box::new(SubtreeT::new(2, Some(h1)));
        let h3 = Box::new(SubtreeT::new(3, Some(h2)));
        let h4 = Box::new(SubtreeT::new(4, Some(h3)));

        let mut container = Vec::<u32>::default();
        let iter = h4.into_iter();
        for h in iter {
            container.push(h.node);
        }
        assert_eq!(container, vec!(4, 3, 2, 1, 0));
    }
}
