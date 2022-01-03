use crate::common::path_iterator::PathIter;
use crate::common::{AsPathIterator, Position};

pub struct PositionIter {
    rightmost_position: Position,
    current_side_node: Option<Position>,
    path_iter: PathIter<Position>,
}

impl PositionIter {
    pub fn new(root: Position, leaf: &Position, leaves_count: u64) -> Self {
        Self {
            rightmost_position: Position::from_leaf_index(leaves_count - 1),
            current_side_node: None,
            path_iter: root.as_path_iter(leaf),
        }
    }
}

impl Iterator for PositionIter {
    type Item = (Position, Position);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((path, mut side)) = self.path_iter.next() {
            if path.in_order_index() <= self.rightmost_position.in_order_index() {
                if self.current_side_node.is_some() {
                    side = self.current_side_node.take().unwrap()
                }
                while side.in_order_index() > self.rightmost_position.in_order_index() {
                    side = side.left_child()
                }
                return Some((path, side));
            } else {
                if self.current_side_node.is_none() {
                    self.current_side_node = Some(side);
                }
            }
        }

        None
    }
}

pub(crate) trait AsPositionIterator {
    fn as_position_iter(&self, leaf: &Position, leaves_count: u64) -> PositionIter;
}

impl AsPositionIterator for Position {
    fn as_position_iter(&self, leaf: &Position, leaves_count: u64) -> PositionIter {
        PositionIter::new(*self, leaf, leaves_count)
    }
}

#[cfg(test)]
mod test {
    use super::AsPositionIterator;
    use crate::common::Position;

    #[test]
    fn test_path_set_returns_path_and_side_nodes_for_4_leaves() {
        //
        //       03
        //      /  \
        //     /    \
        //   01      05
        //  /  \    /  \
        // 00  02  04  06
        // 00  01  02  03

        let root = Position::from_in_order_index(3);

        let leaf = Position::from_leaf_index(0);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 4).unzip();
        let expected_path = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(0),
        ];
        let expected_side = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(2),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(1);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 4).unzip();
        let expected_path = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(2),
        ];
        let expected_side = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(0),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(2);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 4).unzip();
        let expected_path = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(4),
        ];
        let expected_side = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(6),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(3);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 4).unzip();
        let expected_path = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(6),
        ];
        let expected_side = [
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(4),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);
    }

    #[test]
    fn test_path_set_returns_path_and_side_nodes_for_5_leaves() {
        //
        //          07
        //         /  \
        //       03    \
        //      /  \    \
        //     /    \    \
        //   01      05   \
        //  /  \    /  \   \
        // 00  02  04  06  08
        // 00  01  02  03  04

        let root = Position::from_in_order_index(7);

        let leaf = Position::from_leaf_index(0);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 5).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(0),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(8),
            Position::from_in_order_index(5),
            Position::from_in_order_index(2),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(1);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 5).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(2),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(8),
            Position::from_in_order_index(5),
            Position::from_in_order_index(0),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(2);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 5).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(4),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(8),
            Position::from_in_order_index(1),
            Position::from_in_order_index(6),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(3);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 5).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(6),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(8),
            Position::from_in_order_index(1),
            Position::from_in_order_index(4),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(4);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 5).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(8),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);
    }

    #[test]
    fn test_path_set_returns_path_and_side_nodes_for_6_leaves() {
        //
        //            07
        //           /  \
        //          /    \
        //         /      \
        //       03        \
        //      /  \        \
        //     /    \        \
        //   01      05      09
        //  /  \    /  \    /  \
        // 00  02  04  06  08  10
        // 00  01  02  03  04  05

        let root = Position::from_in_order_index(7);

        let leaf = Position::from_leaf_index(0);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(0),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(5),
            Position::from_in_order_index(2),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(1);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(2),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(5),
            Position::from_in_order_index(0),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(2);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(4),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(1),
            Position::from_in_order_index(6),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(3);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(6),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(1),
            Position::from_in_order_index(4),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(4);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(8),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(10),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(5);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 6).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(9),
            Position::from_in_order_index(10),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(8),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);
    }

    #[test]
    fn test_path_set_returns_path_and_side_nodes_for_7_leaves() {
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
        //   01      05      09      \
        //  /  \    /  \    /  \      \
        // 00  02  04  06  08  10     12
        // 00  01  02  03  04  05     06

        let root = Position::from_in_order_index(7);

        let leaf = Position::from_leaf_index(0);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(0),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(5),
            Position::from_in_order_index(2),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(1);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(1),
            Position::from_in_order_index(2),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(5),
            Position::from_in_order_index(0),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(2);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(4),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(1),
            Position::from_in_order_index(6),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(3);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(5),
            Position::from_in_order_index(6),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(1),
            Position::from_in_order_index(4),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(4);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(9),
            Position::from_in_order_index(8),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(12),
            Position::from_in_order_index(10),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(5);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(9),
            Position::from_in_order_index(10),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(12),
            Position::from_in_order_index(8),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);

        let leaf = Position::from_leaf_index(6);
        let (path_positions, side_positions): (Vec<Position>, Vec<Position>) =
            root.as_position_iter(&leaf, 7).unzip();
        let expected_path = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(11),
            Position::from_in_order_index(12),
        ];
        let expected_side = [
            Position::from_in_order_index(7),
            Position::from_in_order_index(3),
            Position::from_in_order_index(9),
        ];
        assert_eq!(path_positions, expected_path);
        assert_eq!(side_positions, expected_side);
    }
}
