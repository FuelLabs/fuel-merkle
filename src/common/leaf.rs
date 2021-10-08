use std::fmt::Debug;

/// #Leaf
///
/// For a given integer type `ux`, where `x` is the number of bits that comprise this integer type,
/// the balanced binary tree that can be represented by indices of type `ux` will have a maximum
/// height of `x`. The maximum number of leaves that such a balanced binary tree will have is
/// 2<sup>x</sup>. A Sparse Merkle Tree, where all leaves are necessarily present, represents a
/// complete binary tree exhibiting the maximum height and maximum number of leaves permissible by
/// its underlying integer type.
///
/// For example, imagine an integer type `u3` that is composed of 3 bits. The complete binary tree
/// represented by this type is the following:
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
/// path needed to traverse from the root of the tree to that leaf.
///
/// E.g., leaf 6
/// ```text
/// 0d6: u3 = 0b110
///         = Right, Right, Left
/// ```
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Leaf(u64);

impl Leaf {
    fn as_u64(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use crate::common::utils::msb_index_from_right;
    use crate::common::Position;

    #[test]
    fn test_path() {
        let mut path = Vec::<Position>::default();

        let mut p = Position::from_in_order_index(7);
        let leaf = 7u64;

        for i in (0..p.height()).rev() {
            let shift = 1 << i;
            let n = (leaf & shift != 0) as u8;
            if n == 0 {
                p = p.left_child();
            } else {
                p = p.right_child();
            }
            path.push(p);
        }

        println!("{:?}", path);
    }
}
