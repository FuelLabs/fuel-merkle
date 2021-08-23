use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    parent: Option<RefNode<T>>,
    left: Option<RefNode<T>>,
    right: Option<RefNode<T>>,
}

type RefNode<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    pub fn new(data: T) -> RefNode<T> {
        Rc::new(RefCell::new( Self {
            data, parent: None, left: None, right: None
        }))
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn set_parent(&mut self, node: Option<RefNode<T>>) {
        self.parent = node;
    }

    pub fn parent(&self) -> Option<RefNode<T>> {
        self.parent.as_ref().map(|node| node.clone())
    }

    pub fn set_left(&mut self, node: Option<RefNode<T>>) {
        self.left = node;
    }

    pub fn left(&self) -> Option<RefNode<T>> {
        self.left.as_ref().map(|node| node.clone())
    }

    pub fn set_right(&mut self, node: Option<RefNode<T>>) {
        self.right = node;
    }

    pub fn right(&self) -> Option<RefNode<T>> {
        self.right.as_ref().map(|node| node.clone())
    }
}

impl<T: Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

#[derive(Debug)]
struct NNode<T> {
    node: RefNode<T>
}

impl<T> NNode<T> {
    pub fn new(data: T) -> Self {
        Self { node: Node::<T>::new(data) }
    }

    pub fn set_parent(&mut self, node: &NNode<T>) {
        self.node.borrow_mut().set_parent(Some(Rc::clone(&node.node)));
    }

    pub fn parent(&self) -> Option<Self> {
        self.node.borrow_mut().parent().map(|node| Self { node })
    }

    pub fn set_left(&mut self, node: &mut NNode<T>) {
        node.set_parent(&self);
        self.node.borrow_mut().set_left(Some(Rc::clone(&node.node)));
    }

    pub fn left(&self) -> Option<Self> {
        self.node.borrow_mut().left().map(|node| Self { node })
    }

    pub fn set_right(&mut self, node: &mut NNode<T>) {
        node.set_parent(&self);
        self.node.borrow_mut().set_right(Some(Rc::clone(&node.node)));
    }

    pub fn right(&self) -> Option<Self> {
        self.node.borrow_mut().right().map(|node| Self { node })
    }
}

// pub struct NodeProofIterator<T> {
//     current: Option<RefNode<T>>,
//     previous: Option<RefNode<T>>,
// }
//
// impl<T> NodeProofIterator<T> {
//     pub fn new(node: RefNode<T>) -> Self {
//         Self {
//             current: Some(Rc::clone(&node)),
//             previous: Some(Rc::clone(&node)),
//         }
//     }
// }
//
// impl<T> Iterator for NodeProofIterator<T> {
//     type Item = RefNode<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let node = self.node.take();
//         node.map(|mut n| {
//             self.node = n.take_next();
//             n
//         })
//     }
// }
//
// impl<T> IntoIterator for Node<T> {
//     type Item = RefCell<Node<T>>;
//     type IntoIter = NodeProofIterator<T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         Self::IntoIter::new(Some)
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_into_iter() {
        type N<'a> = Node<u32>;

        //          08
        //         /  \
        //       07    \
        //      /  \    \
        //     /    \    \
        //   05      06   \
        //  /  \    /  \   \
        // 00  01  02  03  04

        let n00 = N::new(0);
        let n01 = N::new(1);
        let n05 = N::new(5);

        n00.borrow_mut().set_parent(Some(Rc::clone(&n05)));
        n01.borrow_mut().set_parent(Some(Rc::clone(&n05)));
        n05.borrow_mut().set_left(Some(Rc::clone(&n00)));
        n05.borrow_mut().set_right(Some(Rc::clone(&n01)));

        let n02 = N::new(2);
        let n03 = N::new(3);
        let n06 = N::new(6);

        n02.borrow_mut().set_parent(Some(Rc::clone(&n06)));
        n03.borrow_mut().set_parent(Some(Rc::clone(&n06)));
        n06.borrow_mut().set_left(Some(Rc::clone(&n02)));
        n06.borrow_mut().set_right(Some(Rc::clone(&n03)));

        let n07 = N::new(7);
        let n04 = N::new(4);
        let n08 = N::new(8);

        n05.borrow_mut().set_parent(Some(Rc::clone(&n07)));
        n06.borrow_mut().set_parent(Some(Rc::clone(&n07)));
        n07.borrow_mut().set_left(Some(Rc::clone(&n05)));
        n07.borrow_mut().set_right(Some(Rc::clone(&n06)));
        n07.borrow_mut().set_parent(Some(Rc::clone(&n08)));
        n04.borrow_mut().set_parent(Some(Rc::clone(&n08)));
        n08.borrow_mut().set_left(Some(Rc::clone(&n07)));
        n08.borrow_mut().set_right(Some(Rc::clone(&n04)));

        let mut current = Some(n00);
        while current.is_some() {
            println!("{:?}", current);
            let node = current.map(|node| Rc::clone(&node)).unwrap();
            current = node.borrow_mut().parent().map(|n| Rc::clone(&n));
        }
    }

    #[test]
    fn test_nnode() {
        type N<'a> = Node<u32>;

        //          08
        //         /  \
        //       07    \
        //      /  \    \
        //     /    \    \
        //   05      06   \
        //  /  \    /  \   \
        // 00  01  02  03  04

        let mut n00 = NNode::new(0);
        let mut n01 = NNode::new(1);
        let mut n02 = NNode::new(2);
        let mut n03 = NNode::new(3);
        let mut n04 = NNode::new(4);

        let mut n05 = NNode::new(5);
        n05.set_left(&mut n00);
        n05.set_right(&mut n01);

        let mut n06 = NNode::new(6);
        n06.set_left(&mut n02);
        n06.set_right(&mut n03);

        let mut n07 = NNode::new(7);
        n07.set_left(&mut n05);
        n07.set_right(&mut n06);

        let mut n08 = NNode::new(8);
        n08.set_left(&mut n07);
        n08.set_right(&mut n04);

        let mut current = Some(n04);
        while current.is_some() {
            println!("Current: {:?}", current);
            current = current.unwrap().parent().map(|node| node);
        }
    }
}
