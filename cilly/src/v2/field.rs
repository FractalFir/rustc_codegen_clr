use serde::{Deserialize, Serialize};

use super::bimap::BiMapIndex;
use super::{bimap::IntoBiMapIndex, ClassRefIdx, StringIdx, Type};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct FieldIdx(BiMapIndex);
impl IntoBiMapIndex for FieldIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct FieldDesc {
    owner: ClassRefIdx,
    name: StringIdx,
    tpe: Type,
}

impl FieldDesc {
    #[must_use]
    pub fn new(owner: ClassRefIdx, name: StringIdx, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    #[must_use]
    pub fn owner(&self) -> ClassRefIdx {
        self.owner
    }

    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn tpe(&self) -> Type {
        self.tpe
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct StaticFieldIdx(BiMapIndex);
impl IntoBiMapIndex for StaticFieldIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct StaticFieldDesc {
    owner: ClassRefIdx,
    name: StringIdx,
    tpe: Type,
}

impl StaticFieldDesc {
    #[must_use]
    pub fn new(owner: ClassRefIdx, name: StringIdx, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    #[must_use]
    pub fn owner(&self) -> ClassRefIdx {
        self.owner
    }

    #[must_use]
    pub fn name(&self) -> StringIdx {
        self.name
    }

    #[must_use]
    pub fn tpe(&self) -> Type {
        self.tpe
    }
}
