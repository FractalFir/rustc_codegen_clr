use serde::{Deserialize, Serialize};

use super::bimap::BiMapIndex;
use super::Int;
use super::{bimap::IntoBiMapIndex, ClassRefIdx, StringIdx, Type};

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct FieldIdx(BiMapIndex);
impl FieldIdx {
    pub(crate) fn data_ptr(asm: &mut super::Assembly, res: ClassRefIdx) -> FieldIdx {
       let name = asm.alloc_string(crate::DATA_PTR);
       let tpe = asm.nptr(Type::Void);
        asm.alloc_field(FieldDesc { owner: res, name, tpe})
    }
    
    pub(crate) fn metadata(asm: &mut super::Assembly, res: ClassRefIdx) -> FieldIdx {
        let name = asm.alloc_string(crate::METADATA);
        asm.alloc_field(FieldDesc { owner: res, name, tpe:Type::Int(Int::USize)})
    }
}
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
