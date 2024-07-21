use fxhash::{FxHashMap, FxHasher};
use std::{collections::hash_map::Entry, hash::Hash};

pub struct BiMap<Key: HashWrapper + Eq + Hash + Clone, Value: Eq + Hash + Clone>(
    pub FxHashMap<Key, Value>,
    pub FxHashMap<Value, Key>,
);
impl<Key: HashWrapper + Eq + Hash + Clone, Value: Eq + Hash + Clone> Default for BiMap<Key, Value> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<Key: HashWrapper + Eq + Hash + Clone, Value: Eq + Hash + Clone> BiMap<Key, Value> {
    /// Allocates a new Value and returns a Key.
    pub fn alloc(&mut self, val: Value) -> Key {
        match self.1.entry(val.clone()) {
            Entry::Occupied(key) => key.get().clone(),
            Entry::Vacant(empty) => {
                let hash = calculate_hash(&val);
                let key = Key::from_hash(hash);
                empty.insert(key.clone());
                assert!(self.0.insert(key.clone(), val).is_none());
                key
            }
        }
    }
    /// Gets an allocated value with id `key`
    pub fn get(&self, key: Key) -> &Value {
        self.0.get(&key).unwrap()
    }
}
pub trait HashWrapper {
    fn from_hash(val: u64) -> Self;
}
pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}
