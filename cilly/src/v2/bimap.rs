use fxhash::{FxHashMap, FxHasher};
use serde::{Deserialize, Serialize};
use std::{collections::hash_map::Entry, fmt::Debug, hash::Hash, num::NonZeroU32};
#[derive(Serialize, Deserialize, Clone)]
pub struct BiMap<Key, Value: Eq + Hash>(pub Vec<Value>, pub FxHashMap<Value, Key>);
impl<Key: IntoBiMapIndex + Eq + Hash + Clone, Value: Eq + Hash + Clone> Default
    for BiMap<Key, Value>
{
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}
impl<Key: IntoBiMapIndex + Eq + Hash + Clone + Debug, Value: Eq + Hash + Clone + Debug>
    BiMap<Key, Value>
{
    /// Allocates a new Value and returns a Key.
    pub fn alloc(&mut self, val: Value) -> Key {
        match self.1.entry(val.clone()) {
            Entry::Occupied(key) => key.get().clone(),
            Entry::Vacant(empty) => {
                let key = Key::from_index(NonZeroU32::new(self.0.len() as u32 + 1).unwrap());

                empty.insert(key.clone());
                self.0.push(val);
                key
            }
        }
    }
    /// Gets an allocated value with id `key`
    pub fn get(&self, key: Key) -> &Value {
        self.0.get(key.as_bimap_index().get() as usize - 1).unwrap()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
pub type BiMapIndex = NonZeroU32;
pub trait IntoBiMapIndex {
    fn from_index(val: BiMapIndex) -> Self;
    fn as_bimap_index(&self) -> BiMapIndex;
}
pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    use std::hash::Hasher;
    let mut s = FxHasher::default();
    t.hash(&mut s);
    s.finish()
}
