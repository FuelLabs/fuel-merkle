mod test_storage {
    use fuel_storage::Storage;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum StorageError {
        #[error("test error")]
        TestError,
    }

    #[derive(Debug)]
    pub struct StorageMap<Key, Value> {
        phantom_key: PhantomData<Key>,
        phantom_value: PhantomData<Value>,
    }

    impl<Key, Value> StorageMap<Key, Value> {
        pub fn new() -> Self {
            Self {
                phantom_key: PhantomData,
                phantom_value: PhantomData,
            }
        }
    }

    impl<Key, Value> Storage<Key, Value> for StorageMap<Key, Value>
    where
        Value: Clone,
    {
        type Error = StorageError;

        fn insert(&mut self, _: &Key, _: &Value) -> Result<Option<Value>, Self::Error> {
            Err(StorageError::TestError)
        }

        fn remove(&mut self, _: &Key) -> Result<Option<Value>, Self::Error> {
            Err(StorageError::TestError)
        }

        fn get(&self, _: &Key) -> Result<Option<Cow<Value>>, Self::Error> {
            Err(StorageError::TestError)
        }

        fn contains_key(&self, _: &Key) -> Result<bool, Self::Error> {
            Err(StorageError::TestError)
        }
    }
}

use fuel_merkle::binary::{MerkleTree, Node};
use test_storage::StorageMap;

#[test]
pub fn test_push_returns_error_when_storage_returns_error() {
    let mut mock_storage = StorageMap::<[u8; 32], Node<[u8; 32]>>::new();
    let mut tree = MerkleTree::new(&mut mock_storage);
    let push = tree.push(&[0u8; 32]);

    assert!(push.is_err());
}

#[test]
pub fn test_prove_returns_error_when_storage_returns_error() {
    let mut mock_storage = StorageMap::<[u8; 32], Node<[u8; 32]>>::new();
    let mut tree = MerkleTree::new(&mut mock_storage);
    let _ = tree.push(&[0u8; 32]);
    let prove = tree.prove(0);

    assert!(prove.is_err());
}
