use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use interner::global::GlobalPool;
use internment::ArcIntern;

use postcard::fixint::be::deserialize;
use serde::{Deserialize, Serialize};

use crate::utilis::MemoryUsage;
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterCow<T: 'static + Send + Sync + Eq + Hash + Clone> {
    inner: ArcIntern<T>,
}
impl<T: Eq + Sync + Send + Hash + Clone> InterCow<T> {
    pub fn new(val: T) -> Self {
        val.into()
    }
    pub fn borrow_mut(&mut self) -> InterCowRefMut<T> {
        InterCowRefMut(self, self.inner.as_ref().clone())
    }
}

impl<T: std::marker::Send + Hash + Eq + Sync + Debug + Clone> Debug for InterCow<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl<T: std::marker::Send + Hash + Eq + Sync + Clone> From<T> for InterCow<T> {
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}
impl<T: std::marker::Send + Hash + Eq + Sync + Clone> std::ops::Deref for InterCow<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub struct InterCowRefMut<'a, T: 'static + Send + Sync + Eq + Hash + Clone>(&'a mut InterCow<T>, T);
impl<'a, T: 'static + Send + Sync + Eq + Hash + Clone> Deref for InterCowRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}
impl<'a, T: 'static + Send + Sync + Eq + Hash + Clone> DerefMut for InterCowRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
impl<'a, T: 'static + Send + Sync + Eq + Hash + Clone> Drop for InterCowRefMut<'a, T> {
    fn drop(&mut self) {
        *self.0 = self.1.clone().into();
    }
}
impl From<&str> for InterCow<String> {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}
impl Display for InterCow<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
impl AsRef<str> for InterCow<String> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
impl Borrow<str> for InterCow<String> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
impl From<&str> for InterCow<Box<str>> {
    fn from(value: &str) -> Self {
        Self::new(value.into())
    }
}
impl From<String> for InterCow<Box<str>> {
    fn from(value: String) -> Self {
        Self::new(value.into())
    }
}
impl Display for InterCow<Box<str>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}
impl AsRef<str> for InterCow<Box<str>> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
impl Borrow<str> for InterCow<Box<str>> {
    fn borrow(&self) -> &str {
        self
    }
}
#[test]
fn serde_intercow() {
    use ordered_float::OrderedFloat;
    let strcow = InterCow::new(crate::cil_node::CILNode::LdcF32(OrderedFloat(6.7)));
    let bytes = postcard::to_stdvec(&strcow).unwrap();
    let node: InterCow<crate::cil_node::CILNode> =
        postcard::from_bytes(&bytes).expect("ERROR:Could not decode the assembly file!");
    assert_eq!(strcow, node);
    let a1: &crate::cil_node::CILNode = &strcow;
    let a2: &crate::cil_node::CILNode = &node;
    assert_eq!(a1 as *const _ as usize, a2 as *const _ as usize);
    // They ought to have the same address.
}
#[test]
fn intercow_set() {
    let mut strcow = InterCow::new(0);
    *strcow.borrow_mut() = 8;
    assert_eq!(*strcow, 8);
}
/*
// BEFORE MEMORY OPT:
Maximum resident set size (kbytes): 904452
    Average resident set size (kbytes): 0
Maximum resident set size (kbytes): 895348
    Average resident set size (kbytes): 0
Maximum resident set size (kbytes): 895280
    Average resident set size (kbytes): 0

*/

/*
Command being timed: "cargo test --release"
    User time (seconds): 80.10
    System time (seconds): 2.75
    Percent of CPU this job got: 146%
    Elapsed (wall clock) time (h:mm:ss or m:ss): 0:56.43
    Average shared text size (kbytes): 0
    Average unshared data size (kbytes): 0
    Average stack size (kbytes): 0
    Average total size (kbytes): 0
    Maximum resident set size (kbytes): 895636
    Average resident set size (kbytes): 0
    Major (requiring I/O) page faults: 33
    Minor (reclaiming a frame) page faults: 1187608
    Voluntary context switches: 6770
    Involuntary context switches: 1433
    Swaps: 0
    File system inputs: 17328
    File system outputs: 1040544
    Socket messages sent: 0
    Socket messages received: 0
    Signals delivered: 0
    Page size (bytes): 4096
    Exit status: 0 */
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternedString {
    inner: interner::global::GlobalString,
}
impl Serialize for InternedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.as_ref().serialize(serializer)
    }
}
static STRINGS: GlobalPool<String> = GlobalPool::new();
impl<'de> Deserialize<'de> for InternedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tmp: Box<str> = Box::<str>::deserialize(deserializer)?;
        Ok(Self {
            inner: STRINGS.get(tmp.as_ref()),
        })
    }
}
impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(std::convert::AsRef::<str>::as_ref(&*self.inner), f)
    }
}
impl From<String> for InternedString {
    fn from(value: String) -> Self {
        Self {
            inner: STRINGS.get(value),
        }
    }
}
impl From<&str> for InternedString {
    fn from(value: &str) -> Self {
        Self {
            inner: STRINGS.get(value),
        }
    }
}
impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}
impl Borrow<str> for InternedString {
    fn borrow(&self) -> &str {
        &self.inner
    }
}
impl Deref for InternedString {
    fn deref(&self) -> &str {
        &self.inner
    }

    type Target = str;
}
impl MemoryUsage for InternedString {
    fn memory_usage(&self, _: &mut impl crate::utilis::MemoryUsageCounter) -> usize {
        0
    }
}
