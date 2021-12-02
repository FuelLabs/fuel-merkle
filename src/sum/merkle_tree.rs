use fuel_storage::Storage;
use std::borrow::Cow;
use std::marker::PhantomData;

use crate::sum::hash::{empty_sum, leaf_sum, node_sum, Data};
use crate::sum::node::{Node, StorageNode};
use crate::sum::subtree::Subtree;

use crate::common::{AsPathIterator, Bytes32};

#[derive(Debug, thiserror::Error)]
pub enum MerkleTreeError {
    #[error("proof index {0} is not valid")]
    InvalidProofIndex(u64),
}

type DataNode = Node;
type ProofSet = Vec<Data>;

pub struct MerkleTree<'a, 'storage, StorageError> {
    phantom: PhantomData<&'a StorageError>,
    storage: &'storage mut dyn Storage<Data, DataNode, Error = StorageError>,
    head: Option<Box<Subtree<DataNode>>>,
    leaves: Vec<Data>,
    leaves_count: u64,
}

impl<'a, 'storage, StorageError> MerkleTree<'a, 'storage, StorageError>
where
    StorageError: std::error::Error + 'static + Clone,
{
    pub fn new(storage: &'storage mut dyn Storage<Data, DataNode, Error = StorageError>) -> Self {
        Self {
            phantom: PhantomData::default(),
            storage,
            head: None,
            leaves: Vec::<Data>::default(),
            leaves_count: 0,
        }
    }

    pub fn root(&mut self) -> Result<Data, Box<dyn std::error::Error>> {
        let root_node = self.root_node()?;
        let root = match root_node {
            None => *empty_sum(),
            Some(ref node) => node.key(),
        };
        Ok(root)
    }

    pub fn push(&mut self, data: &[u8], fee: u32) -> Result<(), Box<dyn std::error::Error>> {
        let node = {
            let height = 0;
            let leaf_sum = leaf_sum(data);
            DataNode::new(height, leaf_sum, fee)
        };

        self.storage.insert(&node.key(), &node)?;
        self.leaves.push(node.key());

        let next = self.head.take();
        let head = Box::new(Subtree::<DataNode>::new(node, next));
        self.head = Some(head);
        self.join_all_subtrees();

        self.leaves_count += 1;

        Ok(())
    }

    //
    // PRIVATE
    //

    fn root_node(&mut self) -> Result<Option<DataNode>, Box<dyn std::error::Error>> {
        let root_node = match self.head {
            None => None,
            Some(ref initial) => {
                let mut current = initial.clone();
                while current.next().is_some() {
                    let mut head = current;
                    let mut head_next = head.take_next().unwrap();
                    current = self.join_subtrees(&mut head_next, &mut head)?
                }
                Some(current.node().clone())
            }
        };
        Ok(root_node)
    }

    fn join_all_subtrees(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let current = self.head.as_ref().unwrap();
            if !(current.next().is_some()
                && current.node().height() == current.next_node().unwrap().height())
            {
                break;
            }

            // Merge the two front nodes of the list into a single node
            let joined_node = {
                let mut head = self.head.take().unwrap();
                let mut head_next = head.take_next().unwrap();
                self.join_subtrees(&mut head_next, &mut head)?
            };
            self.head = Some(joined_node);
        }

        Ok(())
    }

    fn join_subtrees(
        &mut self,
        lhs: &mut Subtree<DataNode>,
        rhs: &mut Subtree<DataNode>,
    ) -> Result<Box<Subtree<DataNode>>, Box<dyn std::error::Error>> {
        let mut joined_node = {
            let height = lhs.node().height() + 1;
            let fee = lhs.node().fee() + rhs.node().fee();
            let node_sum = node_sum(
                lhs.node().fee(),
                &lhs.node().key(),
                rhs.node().fee(),
                &rhs.node().key(),
            );
            DataNode::new(height, node_sum, fee)
        };

        joined_node.set_left_child_key(Some(lhs.node().hash()));
        joined_node.set_left_child_fee(lhs.node().fee());
        joined_node.set_right_child_key(Some(rhs.node().hash()));
        joined_node.set_right_child_fee(rhs.node().fee());

        self.storage.insert(&joined_node.hash(), &joined_node)?;

        let joined_head = Subtree::new(joined_node, lhs.take_next());
        Ok(Box::new(joined_head))
    }
}

#[cfg(test)]
mod test {
    use fuel_merkle_test_helpers::TEST_DATA;

    use super::{Data, MerkleTree, Node};
    use crate::common::{StorageError, StorageMap};
    use crate::sum::hash::{empty_sum, leaf_sum, node_sum};

    type DataNode = Node;
    type MT<'a, 'storage> = MerkleTree<'a, 'storage, StorageError>;
    const FEE: u32 = 100;

    #[test]
    fn root_returns_the_hash_of_the_empty_string_when_no_leaves_are_pushed() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MT::new(&mut storage_map);

        let root = tree.root().unwrap();
        assert_eq!(root, empty_sum().clone());
    }

    #[test]
    fn root_returns_the_hash_of_the_leaf_when_one_leaf_is_pushed() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MT::new(&mut storage_map);

        let data = &TEST_DATA[0];
        tree.push(&data, FEE);
        let root = tree.root().unwrap();

        let expected = leaf_sum(&data);
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_4_leaves_are_pushed() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MT::new(&mut storage_map);

        let data = &TEST_DATA[0..4]; // 4 leaves
        for datum in data.iter() {
            tree.push(datum, FEE);
        }
        let root = tree.root().unwrap();

        //       N3
        //      /  \
        //     /    \
        //   N1      N2
        //  /  \    /  \
        // L1  L2  L3  L4

        let leaf_1 = leaf_sum(&data[0]);
        let leaf_2 = leaf_sum(&data[1]);
        let leaf_3 = leaf_sum(&data[2]);
        let leaf_4 = leaf_sum(&data[3]);

        let node_1 = node_sum(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_sum(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_sum(FEE * 2, &node_1, FEE * 2, &node_2);

        let expected = node_3;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_5_leaves_are_pushed() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MT::new(&mut storage_map);

        let data = &TEST_DATA[0..5]; // 5 leaves
        for datum in data.iter() {
            tree.push(datum, FEE);
        }
        let root = tree.root().unwrap();

        //          N4
        //         /  \
        //       N3    \
        //      /  \    \
        //     /    \    \
        //   N1      N2   \
        //  /  \    /  \   \
        // L1  L2  L3  L4  L5

        let leaf_1 = leaf_sum(&data[0]);
        let leaf_2 = leaf_sum(&data[1]);
        let leaf_3 = leaf_sum(&data[2]);
        let leaf_4 = leaf_sum(&data[3]);
        let leaf_5 = leaf_sum(&data[4]);

        let node_1 = node_sum(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_sum(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_sum(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_4 = node_sum(FEE * 4, &node_3, FEE * 1, &leaf_5);

        let expected = node_4;
        assert_eq!(root, expected);
    }

    #[test]
    fn root_returns_the_hash_of_the_head_when_7_leaves_are_pushed() {
        let mut storage_map = StorageMap::<Data, DataNode>::new();
        let mut tree = MT::new(&mut storage_map);

        let data = &TEST_DATA[0..7]; // 7 leaves
        for datum in data.iter() {
            tree.push(datum, FEE);
        }
        let root = tree.root().unwrap();

        //              N6
        //          /        \
        //         /          \
        //       N4            N5
        //      /  \           /\
        //     /    \         /  \
        //   N1      N2      N3   \
        //  /  \    /  \    /  \   \
        // L1  L2  L3  L4  L5  L6  L7

        let leaf_1 = leaf_sum(&data[0]);
        let leaf_2 = leaf_sum(&data[1]);
        let leaf_3 = leaf_sum(&data[2]);
        let leaf_4 = leaf_sum(&data[3]);
        let leaf_5 = leaf_sum(&data[4]);
        let leaf_6 = leaf_sum(&data[5]);
        let leaf_7 = leaf_sum(&data[6]);

        let node_1 = node_sum(FEE * 1, &leaf_1, FEE * 1, &leaf_2);
        let node_2 = node_sum(FEE * 1, &leaf_3, FEE * 1, &leaf_4);
        let node_3 = node_sum(FEE * 1, &leaf_5, FEE * 1, &leaf_6);
        let node_4 = node_sum(FEE * 2, &node_1, FEE * 2, &node_2);
        let node_5 = node_sum(FEE * 2, &node_3, FEE * 1, &leaf_7);
        let node_6 = node_sum(FEE * 4, &node_4, FEE * 3, &node_5);

        let expected = node_6;
        assert_eq!(root, expected);
    }
}
