use crate::common::Position;
use std::fmt::Debug;

/// #Leaf
///
/// For a given integer type `ux`, where `x` is the number of bits that compose this integer type,
/// the balanced binary tree that can be represented by indices of type `ux` will have a maximum
/// height of `x`. The maximum number of leaves that such a balanced binary tree will have is
/// 2<sup>x</sup>. A Sparse Merkle Tree, where all leaves are necessarily present, represents a
/// complete binary tree exhibiting the maximum height and maximum number of leaves permissible by
/// its underlying integer type.
///
/// For example, imagine an integer type `u3` that comprises 3 bits. The complete binary tree
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
/// path needed to traverse from the root of the tree to that leaf. The index's binary
/// representation can be read left to right as a sequence of traversal instructions: a 0 bit means
/// "descend left" and a 1 bit means "descend right". By following the `x` bits composing the index,
/// starting at the root, descending to the left child at each `0`, descending to the right child at
/// each `1`, we arrive at the leaf position, having touched every node position along the path
/// formed by this index.
///
/// Alternatively, this can be interpreted as reading the index's most significant bit (MSB) at an
/// offset `n`: read the `n`th bit to the right of the MSB. Here, `n` is a given step in the tree
/// traversal, starting at 0, and incrementing by 1 at each depth until the leaf is reached.
/// The traversal path is then the list of nodes calculated by traversing the tree using the
/// instruction (`0` or `1`) indicated at `x`<sub>`n`</sub>, where `x` is the index in binary
/// representation, and `n` is the offset for each digit in `x` from the MSB.
///
/// Reversing this path gives us the path from the leaf to the root.
///
/// For example, imagine again our integer type `u3` underpinning our tree indices, and a given
/// leaf with leaf index `6`. In the above diagram, this is the seventh leaf in the leaf layer. The
/// path from the root to this leaf is represented by the following list of indices: `07, 11, 13,
/// 06`.
///
/// ```text
/// 0d6: u3 = 0b110
///         = Right, Right, Left
/// ```
///
/// Starting at the tree's root at index `07`, we can follow the instructions encoded by the binary
/// representation of leaf `06`:
/// 1. The first bit is `1`; move right from `07` to `11`.
/// 2. The next bit is `1`; move right from `11` to `13`.
/// 3. The next and final bit is `0`; move left from `13` to `12`.
///
/// We have arrived at the desired leaf position with in-order index `12` and leaf index `06`.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Leaf(Position);

impl Leaf {
    fn as_u64(self) -> u64 {
        self
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
        let leaf = 3u64;

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
