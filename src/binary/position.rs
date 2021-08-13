#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position(u64);

impl Position {
    pub fn from_index(index: u64) -> Self {
        Position(index)
    }

    pub fn from_leaf_index(index: u64) -> Self {
        Position(index * 2)
    }

    pub fn value(self) -> u64 {
        self.0
    }

    /// The sibling position.
    /// A position shares the same parent and height as its sibling.
    pub fn sibling(&self) -> Self {
        let shift = 1 << (self.height() + 1);
        let index = self.value() as i64 + shift * self.direction();
        Self(index as u64)
    }

    /// The parent position.
    /// The parent position has a height less 1 relative to this position.
    pub fn parent(&self) -> Self {
        let shift = 1 << self.height();
        let index = self.value() as i64 + shift * self.direction();
        Self(index as u64)
    }

    /// The uncle position.
    /// The uncle position is the sibling of the parent and has a height less 1 relative to this position.
    pub fn uncle(&self) -> Self {
        self.parent().sibling()
    }

    /// The height of the index in a binary tree.
    /// Leaf nodes represent height 0. A leaf's parent represents height 1.
    /// Height values monotonically increase as you ascend the tree.
    pub fn height(&self) -> u32 {
        (!self.value()).trailing_zeros()
    }

    // PRIVATE

    /// Orientation of the position index relative to its parent.
    /// Returns 0 if the index is left of its parent.
    /// Returns 1 if the index is right of its parent.
    fn orientation(&self) -> u8 {
        let shift = 1 << (self.height() + 1);
        (self.value() & shift != 0) as u8
    }

    /// The "direction" to travel to reach the parent node.
    /// Returns +1 if the index is left of its parent.
    /// Returns -1 if the index is right of its parent.
    fn direction(&self) -> i64 {
        let scale = self.orientation() as i64 * 2 - 1; // Scale [0, 1] to [-1, 1];
        -scale
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_index() {
        assert_eq!(Position::from_index(0).value(), 0);
        assert_eq!(Position::from_index(1).value(), 1);
        assert_eq!(Position::from_index(!0u64).value(), !0u64);
    }

    #[test]
    fn test_from_leaf_index() {
        assert_eq!(Position::from_leaf_index(0).value(), 0);
        assert_eq!(Position::from_leaf_index(1).value(), 2);
        assert_eq!(Position::from_leaf_index((!0u64) >> 1).value(), !0u64 - 1);
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
