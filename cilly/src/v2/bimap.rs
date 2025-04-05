use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::Entry, fmt::Debug, hash::Hash, marker::PhantomData, num::NonZeroU32,
    ops::Index,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BiMap<Key, Value: Eq + Hash>(pub Vec<Value>, pub FxHashMap<Value, Key>);
impl<Key: IntoBiMapIndex + Eq + Hash + Clone, Value: Eq + Hash + Clone> Default
    for BiMap<Key, Value>
{
    fn default() -> Self {
        Self(Vec::default(), FxHashMap::default())
    }
}
impl<Key: IntoBiMapIndex + Eq + Hash + Clone + Debug, Value: Eq + Hash + Clone + Debug> Index<Key>
    for BiMap<Key, Value>
{
    type Output = Value;

    fn index(&self, index: Key) -> &Self::Output {
        self.get(index)
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
                let key = Key::from_index(
                    NonZeroU32::new(u32::try_from(self.0.len()).expect("Key ID out of range") + 1)
                        .expect(
                            "Key ID 0 when a non-zero value expected, this could be an overflow",
                        ),
                );

                empty.insert(key.clone());
                self.0.push(val);
                key
            }
        }
    }
    /// Gets an allocated value with id `key`
    // Key is tiny(32 or 64 bit), so passing it by value makes sense
    #[allow(clippy::needless_pass_by_value)]
    pub fn get(&self, key: Key) -> &Value {
        self.0.get(key.as_bimap_index().get() as usize - 1).unwrap()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contais_val(&self, def: Value) -> bool {
        self.1.contains_key(&def)
    }
    pub fn iter_keys(&self) -> impl Iterator<Item = Key> {
        (1..(self.0.len() as u32)).map(|key| Key::from_index(NonZeroU32::new(key).unwrap()))
    }

    pub fn map_values(&mut self, map: impl Fn(&mut Value)) {
        self.0.iter_mut().for_each(&map);
        self.1 = self
            .1
            .iter()
            .map(|(value, key)| {
                let mut value = value.clone();
                map(&mut value);
                (value, key.clone())
            })
            .collect();
    }
}
pub type BiMapIndex = NonZeroU32;
pub trait IntoBiMapIndex {
    fn from_index(val: BiMapIndex) -> Self;
    fn as_bimap_index(&self) -> BiMapIndex;
}
#[test]
fn bimap_alloc() {
    use super::StringIdx;
    use crate::IString;
    let mut map = BiMap::<StringIdx, IString>::default();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    let hi = map.alloc("Hi".into());
    assert!(!map.is_empty());
    assert_eq!(**map.get(hi), *"Hi");
    assert_eq!(map.len(), 1);
    let bob = map.alloc("Bob".into());
    assert_ne!(hi, bob);
    assert_eq!(**map.get(bob), *"Bob");
    assert_eq!(map.len(), 2);
    assert!(!map.is_empty());
}
#[derive(Hash, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Interned<T> {
    pd: PhantomData<T>,
    idx: BiMapIndex,
}
impl<T> Copy for Interned<T> {}
impl<T> Clone for Interned<T> {
    fn clone(&self) -> Self {
        Self {
            pd: self.pd.clone(),
            idx: self.idx.clone(),
        }
    }
}
impl<T> IntoBiMapIndex for Interned<T> {
    fn from_index(idx: BiMapIndex) -> Self {
        Self {
            pd: PhantomData,
            idx,
        }
    }

    fn as_bimap_index(&self) -> BiMapIndex {
        self.idx
    }
}
