use fxhash::{FxHashMap, FxHasher};
use serde::{Deserialize, Serialize};
use std::{collections::hash_map::Entry, fmt::Debug, hash::Hash};
#[derive(Serialize, Deserialize)]
pub struct BiMap<Key: HashWrapper + Eq + Hash + Clone, Value: Eq + Hash + Clone>(
    pub FxHashMap<Key, Value>,
    pub FxHashMap<Value, Key>,
);
impl<Key: HashWrapper + Eq + Hash + Clone, Value: Eq + Hash + Clone> Default for BiMap<Key, Value> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<Key: HashWrapper + Eq + Hash + Clone + Debug, Value: Eq + Hash + Clone + Debug>
    BiMap<Key, Value>
{
    /// Allocates a new Value and returns a Key.
    pub fn alloc(&mut self, val: Value) -> Key {
        match self.1.entry(val.clone()) {
            Entry::Occupied(key) => key.get().clone(),
            Entry::Vacant(empty) => {
                let hash = calculate_hash(&val);
                let key = Key::from_hash(hash);
                empty.insert(key.clone());
                if let Some(collision) = self.0.insert(key.clone(), val.clone()) {
                    panic!("{val:?} and {collision:?} have colliding hashes");
                }

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
