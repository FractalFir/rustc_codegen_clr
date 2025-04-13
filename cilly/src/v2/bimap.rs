use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::Entry, fmt::Debug, hash::Hash, marker::PhantomData, num::NonZeroU32,
    ops::Index,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct BiMap<Value: Eq + Hash>(pub Vec<Value>, pub FxHashMap<Value, Interned<Value>>);
impl<Value: Eq + Hash + Clone> Default for BiMap<Value> {
    fn default() -> Self {
        Self(Vec::default(), FxHashMap::default())
    }
}
impl<Value: Eq + Hash + Clone + Debug> Index<Interned<Value>> for BiMap<Value> {
    type Output = Value;

    fn index(&self, index: Interned<Value>) -> &Self::Output {
        self.get(index)
    }
}

impl<Value: Eq + Hash + Clone + Debug> BiMap<Value> {
    /// Allocates a new Value and returns a Interned<Value>.
    pub fn alloc(&mut self, val: Value) -> Interned<Value> {
        match self.1.entry(val.clone()) {
            Entry::Occupied(key) => key.get().clone(),
            Entry::Vacant(empty) => {
                let key = Interned::from_index(
                    NonZeroU32::new(u32::try_from(self.0.len()).expect("Interned<Value> ID out of range") + 1)
                        .expect(
                            "Interned<Value> ID 0 when a non-zero value expected, this could be an overflow",
                        ),
                );

                empty.insert(key.clone());
                self.0.push(val);
                key
            }
        }
    }
    /// Gets an allocated value with id `key`
    // Interned<Value> is tiny(32 or 64 bit), so passing it by value makes sense
    #[allow(clippy::needless_pass_by_value)]
    pub fn get(&self, key: Interned<Value>) -> &Value {
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
    pub fn iter_keys(&self) -> impl Iterator<Item = Interned<Value>> {
        (1..(self.0.len() as u32)).map(|key| Interned::from_index(NonZeroU32::new(key).unwrap()))
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
    use crate::IString;
    let mut map = BiMap::<IString>::default();
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
pub struct Interned<T: ?Sized> {
    pd: PhantomData<T>,
    idx: BiMapIndex,
}
impl<T: ?Sized> Copy for Interned<T> {}

impl<T> Interned<T> {
    pub fn inner(&self) -> u32 {
        self.idx.get()
    }
}
impl<T: ?Sized> Clone for Interned<T> {
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
