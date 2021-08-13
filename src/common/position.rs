#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position(u64);

impl Position {
    pub fn index(self) -> u64 {
        self.0
    }

    /// Construct a position from an in-order index.
    pub fn from_index(index: u64) -> Self {
        Position(index)
    }

    /// Construct a position from a leaf index. The in-order index corresponding to the leaf index
    /// will always equal the leaf index multiplied by 2.
    pub fn from_leaf_index(index: u64) -> Self {
        Position(index * 2)
    }

    /// The sibling position.
    /// A position shares the same parent and height as its sibling.
    pub fn sibling(self) -> Self {
        let shift = 1 << (self.height() + 1);
        let index = self.index() as i64 + shift * self.direction();
        Self(index as u64)
    }

    /// The parent position.
    /// The parent position has a height less 1 relative to this position.
    pub fn parent(self) -> Self {
        let shift = 1 << self.height();
        let index = self.index() as i64 + shift * self.direction();
        Self(index as u64)
    }

    /// The uncle position.
    /// The uncle position is the sibling of the parent and has a height less 1 relative to this
    /// position.
    pub fn uncle(self) -> Self {
        self.parent().sibling()
    }

    /// The height of the index in a binary tree.
    /// Leaf nodes represent height 0. A leaf's parent represents height 1.
    /// Height values monotonically increase as you ascend the tree.
    ///
    /// Height is deterministically calculated as the number of trailing zeros of the complement of
    /// the position's index. The following table demonstrates the relationship between a position's
    /// height and the trailing zeros.
    ///
    /// | Index (Dec) | Index (Bin) | !Index (Bin) | Trailing 0s | Height |
    /// |-------------|-------------|--------------| ------------|--------|
    /// |           0 |        0000 |         1111 |           0 |      0 |
    /// |           2 |        0010 |         1101 |           0 |      0 |
    /// |           4 |        0100 |         1011 |           0 |      0 |
    /// |           1 |        0001 |         1110 |           1 |      1 |
    /// |           5 |        0101 |         1010 |           1 |      1 |
    /// |           9 |        1001 |         0110 |           1 |      1 |
    /// |           3 |        0011 |         1100 |           2 |      2 |
    /// |          11 |        1011 |         0100 |           2 |      2 |
    ///
    pub fn height(self) -> u32 {
        (!self.index()).trailing_zeros()
    }

    // PRIVATE

    /// Orientation of the position index relative to its parent.
    /// Returns 0 if the index is left of its parent.
    /// Returns 1 if the index is right of its parent.
    ///
    /// The orientation is determined by the reading the `n`th rightmost digit of the index's binary
    /// value, where `n` = the height of the index + 1. The following table demonstrates the
    /// relationships between a position's index, height, and orientation.
    ///
    /// | Index (Dec) | Index (Bin) | Height | Orientation |
    /// |-------------|-------------|--------|-------------|
    /// |           0 |        0000 |      0 |           0 |
    /// |           2 |        0010 |      0 |           1 |
    /// |           4 |        0100 |      0 |           0 |
    /// |           6 |        0110 |      0 |           1 |
    /// |           1 |        0001 |      1 |           0 |
    /// |           5 |        0101 |      1 |           1 |
    /// |           9 |        1001 |      1 |           0 |
    /// |          13 |        1101 |      1 |           1 |
    ///
    fn orientation(self) -> u8 {
        let shift = 1 << (self.height() + 1);
        (self.index() & shift != 0) as u8
    }

    /// The "direction" to travel to reach the parent node.
    /// Returns +1 if the index is left of its parent.
    /// Returns -1 if the index is right of its parent.
    fn direction(self) -> i64 {
        let scale = self.orientation() as i64 * 2 - 1; // Scale [0, 1] to [-1, 1];
        -scale
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_index() {
        assert_eq!(Position::from_index(0).index(), 0);
        assert_eq!(Position::from_index(1).index(), 1);
        assert_eq!(Position::from_index(!0u64).index(), !0u64);
    }

    #[test]
    fn test_from_leaf_index() {
        assert_eq!(Position::from_leaf_index(0).index(), 0);
        assert_eq!(Position::from_leaf_index(1).index(), 2);
        assert_eq!(Position::from_leaf_index((!0u64) >> 1).index(), !0u64 - 1);
    }

    #[test]
    fn test_equality_returns_true_for_two_equal_positions() {
        assert_eq!(Position(0), Position(0));
        assert_eq!(Position::from_index(0), Position(0));
        assert_eq!(Position::from_leaf_index(1), Position(2));
    }

    #[test]
    fn test_equality_returns_false_for_two_unequal_positions() {
        assert_ne!(Position(0), Position(1));
        assert_ne!(Position::from_index(0), Position(1));
        assert_ne!(Position::from_leaf_index(0), Position(2));
    }

    #[test]
    fn test_height() {
        assert_eq!(Position(0).height(), 0);
        assert_eq!(Position(2).height(), 0);
        assert_eq!(Position(4).height(), 0);

        assert_eq!(Position(1).height(), 1);
        assert_eq!(Position(5).height(), 1);
        assert_eq!(Position(9).height(), 1);

        assert_eq!(Position(3).height(), 2);
        assert_eq!(Position(11).height(), 2);
        assert_eq!(Position(19).height(), 2);
    }

    #[test]
    fn test_sibling() {
        assert_eq!(Position(0).sibling(), Position(2));
        assert_eq!(Position(2).sibling(), Position(0));

        assert_eq!(Position(1).sibling(), Position(5));
        assert_eq!(Position(5).sibling(), Position(1));

        assert_eq!(Position(3).sibling(), Position(11));
        assert_eq!(Position(11).sibling(), Position(3));
    }

    #[test]
    fn test_parent() {
        assert_eq!(Position(0).parent(), Position(1));
        assert_eq!(Position(2).parent(), Position(1));

        assert_eq!(Position(1).parent(), Position(3));
        assert_eq!(Position(5).parent(), Position(3));

        assert_eq!(Position(3).parent(), Position(7));
        assert_eq!(Position(11).parent(), Position(7));
    }

    #[test]
    fn test_uncle() {
        assert_eq!(Position(0).uncle(), Position(5));
        assert_eq!(Position(2).uncle(), Position(5));
        assert_eq!(Position(4).uncle(), Position(1));
        assert_eq!(Position(6).uncle(), Position(1));

        assert_eq!(Position(1).uncle(), Position(11));
        assert_eq!(Position(5).uncle(), Position(11));
        assert_eq!(Position(9).uncle(), Position(3));
        assert_eq!(Position(13).uncle(), Position(3));
    }
}
