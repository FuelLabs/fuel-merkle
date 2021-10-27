use crate::common::node::ParentNode;
use crate::common::MSB;

/// #Path Iterator
///
/// For a given integer type `ux`, where `x` is the number of bits that compose this integer type,
/// the balanced binary tree that can be represented by indices of type `ux` will have a maximum
/// height of `x`. The maximum number of leaves that such a balanced binary tree will have is
/// 2<sup>x</sup>. A Sparse Merkle Tree, where all leaves are necessarily present, represents a
/// complete binary tree exhibiting the maximum height and maximum number of leaves permissible by
/// its underlying integer type.
///
/// For example, imagine an integer type `u3` that comprises 3 bits. The complete binary tree
/// represented by this type is the following (indices in this tree are calculated using in-order
/// traversal; see [`Position`](crate::common::Position)):
///
/// ```text
///               07
///              /  \
///             /    \
///            /      \
///           /        \
///          /          \
///         /            \
///       03              11
///      /  \            /  \
///     /    \          /    \
///   01      05      09      13
///  /  \    /  \    /  \    /  \
/// 00  02  04  06  08  10  12  14
/// 00  01  02  03  04  05  06  07
/// ```
///
/// This tree exhibits the following properties:
/// - It has a height of 3 (N.B. leaves have a height 0)
/// - It has 2<sup>3</sup> = 8 leaves
///
/// A naturally arising property of complete binary trees is that a leaf index encodes the unique
/// path needed to traverse from the root of the tree to that leaf. The index's binary
/// representation can be read left to right as a sequence of traversal instructions: a 0 bit means
/// "descend left" and a 1 bit means "descend right". By following the `x` bits composing the index,
/// starting at the root, descending to the left child at each `0`, descending to the right child at
/// each `1`, we arrive at the leaf position, having touched every node position along the path
/// formed by this index. Note that this algorithm does not prescribe how to descend from one node
/// to the next; it describes merely the direction in which to descend at each step.
///
/// Alternatively, this can be interpreted as reading the index's most significant bit (MSB) at an
/// offset `n`: read the `n`th bit to the right of the MSB. Here, `n` is a given step in the tree
/// traversal, starting at 0, and incrementing by 1 at each depth until the leaf is reached. The
/// traversal path is then the list of nodes calculated by traversing the tree using the instruction
/// (`0` or `1`) indicated at `x`<sub>`n`</sub>, where `x` is the index in binary representation,
/// and `n` is the offset for each digit in `x` from the MSB.
///
/// Reversing this path gives us the path from the leaf to the root.
///
/// For example, imagine again our integer type `u3` underpinning our tree indices, and a given
/// leaf with leaf index `6`. In the above diagram, this is the seventh leaf in the leaf layer.
/// Indices in this tree are calculated using in-order traversal. In-order indexing provides a
/// deterministic way to descend from one node to the next. A priori, we can see that the path from
/// the root to this leaf is represented by the following list of in-order indices: `07, 11, 13, 12`
/// (N.B. the leaf index that corresponds to the in-order index `12` is `6`).
///
/// ```text
/// 0d6: u3 = 0b110
///         = Right, Right, Left
/// ```
///
/// Starting at the tree's root at index `07`, we can follow the instructions encoded by the binary
/// representation of leaf `06` (`0b110`). In combination with our in-order index rules for
/// descending nodes, we evaluate the following:
/// 1. The first bit is `1`; move right from `07` to `11`.
/// 2. The next bit is `1`; move right from `11` to `13`.
/// 3. The next and final bit is `0`; move left from `13` to `12`.
///
/// We have arrived at the desired leaf position with in-order index `12` and leaf index `06`.
/// Indeed, following the instructions at each bit has produced the same list of positional indices
/// that we observed earlier: `07, 11, 13, 12`.
///
pub struct PathIter<T> {
    leaf: T,
    current: Option<(T, T)>,
    current_offset: usize,
}

// Height Depth
// 7      0
//                                /
// ...                          ...
//                              /
// 3      4                    07
//                            /  \
//                           /    \
//                          /      \
//                         /        \
//                        /          \
//                       /            \
// 2      5            03              11
//                    /  \            /  \
//                   /    \          /    \
// 1      6        01      05      09      13
//                /  \    /  \    /  \    /  \
// 0      7      00  02  04  06  08  10  12  14
//               00  01  02  03  04  05  06  07
impl<T> PathIter<T>
where
    T: ParentNode + Clone,
{
    pub fn new(root: T, leaf: T) -> Self {
        let initial = (root.clone(), root.clone());
        let initial_offset = T::key_size_in_bits() - T::max_height();
        Self {
            leaf,
            current: Some(initial),
            current_offset: initial_offset,
        }
    }
}

impl<T> Iterator for PathIter<T>
where
    T: ParentNode + Clone,
    T::Key: MSB,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current.clone();

        if let Some(ref path_node_side_node) = self.current {
            let path_node = &path_node_side_node.0;
            if !path_node.is_leaf() {
                let key = self.leaf.key();
                let instruction = key.get_bit_at_index_from_msb(self.current_offset);
                if instruction == 0 {
                    let next = (path_node.left_child(), path_node.right_child());
                    self.current = Some(next);
                } else {
                    let next = (path_node.right_child(), path_node.left_child());
                    self.current = Some(next);
                }
                self.current_offset += 1;
            } else {
                self.current = None;
            }
        }

        value
    }
}

pub trait AsPathIterator<T> {
    fn as_path_iter(&self, leaf: &Self) -> PathIter<T>;
}

impl<T> AsPathIterator<T> for T
where
    T: ParentNode + Clone,
{
    fn as_path_iter(&self, leaf: &Self) -> PathIter<T> {
        PathIter::new(self.clone(), leaf.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::common::{AsPathIterator, Bytes1, Node, ParentNode};

    #[derive(Debug, Clone, PartialEq)]
    struct TestNode<const MAX_HEIGHT: usize> {
        value: u8,
    }

    impl<const MAX_HEIGHT: usize> TestNode<MAX_HEIGHT> {
        pub fn in_order_index(&self) -> u8 {
            self.value
        }

        pub fn leaf_index(&self) -> u8 {
            assert!(self.is_leaf());
            self.value / 2
        }

        pub fn from_in_order_index(index: u8) -> Self {
            Self { value: index }
        }
        pub fn from_leaf_index(index: u8) -> Self {
            Self { value: index * 2 }
        }

        pub fn height(&self) -> u32 {
            (!self.in_order_index()).trailing_zeros()
        }

        pub fn is_leaf(&self) -> bool {
            self.in_order_index() % 2 == 0
        }

        fn child(&self, direction: i8) -> Self {
            assert!(!self.is_leaf());
            let shift = 1 << (self.height() - 1);
            let index = self.in_order_index() as i8 + shift * direction;
            Self::from_in_order_index(index as u8)
        }
    }

    impl<const MAX_HEIGHT: usize> Node for TestNode<MAX_HEIGHT> {
        type Key = Bytes1;

        fn max_height() -> usize {
            MAX_HEIGHT
        }

        fn key(&self) -> Self::Key {
            TestNode::leaf_index(self).to_be_bytes()
        }

        fn is_leaf(&self) -> bool {
            TestNode::is_leaf(self)
        }
    }

    impl<const MAX_HEIGHT: usize> ParentNode for TestNode<MAX_HEIGHT> {
        fn left_child(&self) -> Self {
            TestNode::child(self, -1)
        }

        fn right_child(&self) -> Self {
            TestNode::child(self, 1)
        }
    }

    #[test]
    fn test_path_iter_returns_path() {
        //
        //               07
        //              /  \
        //             /    \
        //            /      \
        //           /        \
        //          /          \
        //         /            \
        //       03              11
        //      /  \            /  \
        //     /    \          /    \
        //   01      05      09      13
        //  /  \    /  \    /  \    /  \
        // 00  02  04  06  08  10  12  14
        // 00  01  02  03  04  05  06  07
        //
        type Node = TestNode<3>;
        let root = Node::from_in_order_index(7);

        {
            let leaf = Node::from_leaf_index(0);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),
                Node::from_in_order_index(1),
                Node::from_in_order_index(0),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(1);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),
                Node::from_in_order_index(1),
                Node::from_in_order_index(2),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(2);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),
                Node::from_in_order_index(5),
                Node::from_in_order_index(4),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(3);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),
                Node::from_in_order_index(5),
                Node::from_in_order_index(6),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(4);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11),
                Node::from_in_order_index(9),
                Node::from_in_order_index(8),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(5);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11),
                Node::from_in_order_index(9),
                Node::from_in_order_index(10),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(6);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11),
                Node::from_in_order_index(13),
                Node::from_in_order_index(12),
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(7);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11),
                Node::from_in_order_index(13),
                Node::from_in_order_index(14),
            ];
            assert_eq!(path, expected_path);
        }
    }

    #[test]
    fn test_path_iter_returns_side_nodes() {
        //
        //               07
        //              /  \
        //             /    \
        //            /      \
        //           /        \
        //          /          \
        //         /            \
        //       03              11
        //      /  \            /  \
        //     /    \          /    \
        //   01      05      09      13
        //  /  \    /  \    /  \    /  \
        // 00  02  04  06  08  10  12  14
        // 00  01  02  03  04  05  06  07
        //
        type Node = TestNode<3>;
        let root = Node::from_in_order_index(7);

        {
            let leaf = Node::from_leaf_index(0);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11), // Sibling node of 3
                Node::from_in_order_index(5),  // Sibling node of 1
                Node::from_in_order_index(2),  // Sibling node of 0
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(1);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11), // Sibling node of 3
                Node::from_in_order_index(5),  // Sibling node of 1
                Node::from_in_order_index(0),  // Sibling node of 2
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(2);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11), // Sibling node of 3
                Node::from_in_order_index(1),  // Sibling node of 5
                Node::from_in_order_index(6),  // Sibling node of 4
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(3);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(11), // Sibling node of 3
                Node::from_in_order_index(1),  // Sibling node of 5
                Node::from_in_order_index(4),  // Sibling node of 6
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(4);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),  // Sibling node of 11
                Node::from_in_order_index(13), // Sibling node of 9
                Node::from_in_order_index(10), // Sibling node of 8
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(5);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),  // Sibling node of 11
                Node::from_in_order_index(13), // Sibling node of 9
                Node::from_in_order_index(8),  // Sibling node of 10
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(6);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),  // Sibling node of 11
                Node::from_in_order_index(9),  // Sibling node of 3
                Node::from_in_order_index(14), // Sibling node of 12
            ];
            assert_eq!(path, expected_path);
        }

        {
            let leaf = Node::from_leaf_index(7);
            let iter = root.as_path_iter(&leaf).map(|pair| pair.1);
            let path: Vec<Node> = iter.collect();
            let expected_path = vec![
                Node::from_in_order_index(7),
                Node::from_in_order_index(3),  // Sibling node of 11
                Node::from_in_order_index(9),  // Sibling node of 3
                Node::from_in_order_index(12), // Sibling node of 14
            ];
            assert_eq!(path, expected_path);
        }
    }

    #[test]
    fn test_path_iter_height_4() {
        //
        //                               15
        //                              /  \
        //                             /    \
        //                            /      \
        //                           /        \
        //                          /          \
        //                         /            \
        //                        /              \
        //                       /                \
        //                      /                  \
        //                     /                    \
        //                    /                      \
        //                   /                        \
        //                  /                          \
        //                 /                            \
        //               07                              23
        //              /  \                            /  \
        //             /    \                          /    \
        //            /      \                        /      \
        //           /        \                      /        \
        //          /          \                    /          \
        //         /            \                  /            \
        //       03              11              19              27
        //      /  \            /  \            /  \            /  \
        //     /    \          /    \          /    \          /    \
        //   01      05      09      13      17      21      25      29
        //  /  \    /  \    /  \    /  \    /  \    /  \    /  \    /  \
        // 00  02  04  06  08  10  12  14  16  18  20  22  24  26  28  30
        // 00  01  02  03  04  05  06  07  08  09  10  11  12  13  14  15
        //
        type Node = TestNode<4>;
        let root = Node::from_in_order_index(15);
        let leaf = Node::from_leaf_index(4);

        let iter = root.as_path_iter(&leaf).map(|pair| pair.0);
        let path: Vec<Node> = iter.collect();

        let expected_path = vec![
            Node::from_in_order_index(15),
            Node::from_in_order_index(7),
            Node::from_in_order_index(11),
            Node::from_in_order_index(9),
            Node::from_in_order_index(8),
        ];
        assert_eq!(path, expected_path);
    }
}
