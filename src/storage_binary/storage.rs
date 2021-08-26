pub trait Storage<Key, Value> {
    // CRD interface
    fn create(&mut self, key: Key, data: Value);

    fn get_all(&self) -> Vec<(Key, Value)>;

    fn get(&self, key: Key) -> Option<Value>;

    fn delete(&mut self, key: Key);
}
