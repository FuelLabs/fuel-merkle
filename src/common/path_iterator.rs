use crate::common::get_bit_at_index_from_msb_u64;
use crate::common::node::ParentNode;

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
    current: Option<T>,
}

impl<T> PathIter<T>
where
    T: ParentNode + Clone,
{
    pub fn new(leaf: T, root: T) -> Self {
        assert!(root.is_ancestor_of(&leaf));
        Self {
            leaf,
            current: Some(root),
        }
    }
}

impl<T> Iterator for PathIter<T>
where
    T: ParentNode + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.current.clone();

        if let Some(ref current) = self.current {
            if !current.is_leaf() {
                let index = self.leaf.index();
                let height = current.height();
                let n = get_bit_at_index_from_msb_u64(index, height);
                if n == 0 {
                    self.current = Some(current.left_child());
                } else {
                    self.current = Some(current.right_child());
                }
            } else {
                self.current = None;
            }
        }

        value
    }
}

pub trait IntoPathIterator<T> {
    fn into_path_iter(self, root: &Self) -> PathIter<T>;
}

impl<T> IntoPathIterator<T> for T
where
    T: ParentNode + Clone,
{
    fn into_path_iter(self, root: &Self) -> PathIter<T> {
        PathIter::new(self, root.clone())
    }
}

#[cfg(test)]
mod test {
    use crate::common::path_iterator::IntoPathIterator;
    use crate::common::Position;

    #[test]
    fn test_path_iterator_returns_path_from_root_to_leaf() {
        let root = Position::from_in_order_index(7);
        let leaf = Position::from_leaf_index(4);
        let iter = leaf.into_path_iter(&root);
        let path: Vec<Position> = iter.collect();

        let expected_path = vec![
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(9),
            Position::from_in_order_index(8),
        ];
        assert_eq!(path, expected_path)
    }

    #[test]
    fn test_path_iterator_returns_path_from_root_to_leaf_in_subtree() {
        let root = Position::from_in_order_index(11);
        let leaf = Position::from_leaf_index(4);
        let iter = leaf.into_path_iter(&root);
        let path: Vec<Position> = iter.collect();

        let expected_path = vec![
            Position::from_in_order_index(11),
            Position::from_in_order_index(9),
            Position::from_in_order_index(8),
        ];
        assert_eq!(path, expected_path)
    }

    #[test]
    #[should_panic]
    fn test_path_iterator_panics_if_leaf_is_not_a_descendent_of_root() {
        let root = Position::from_in_order_index(11);
        let leaf = Position::from_leaf_index(3);
        // This call should panic because `leaf` is not a descendent of `root`
        leaf.into_path_iter(&root);
    }
}
