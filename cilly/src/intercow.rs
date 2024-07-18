use std::{fmt::Debug, hash::Hash};

use internment::ArcIntern;

use serde::{Deserialize, Serialize};

pub struct InterCow<T: 'static + Send + Sync + Eq + Hash> {
    inner: ArcIntern<T>,
}
impl<T: std::cmp::Eq + std::marker::Sync + std::marker::Send + std::hash::Hash> InterCow<T> {
    pub fn new(val: T) -> Self {
        val.into()
    }
}
impl<T: Serialize + std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync> Eq
    for InterCow<T>
{
}
impl<T: Serialize + std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync>
    PartialEq for InterCow<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl<T: Serialize + std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync>
    Serialize for InterCow<T>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}
impl<
        'de,
        T: Deserialize<'de> + std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync,
    > Deserialize<'de> for InterCow<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: ArcIntern::deserialize(deserializer)?,
        })
    }
}
impl<T: std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync + Debug> Debug
    for InterCow<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
impl<T: std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync> From<T>
    for InterCow<T>
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}
impl<T: std::marker::Send + std::hash::Hash + std::cmp::Eq + std::marker::Sync> std::ops::Deref
    for InterCow<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
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
