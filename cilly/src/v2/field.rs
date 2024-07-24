use serde::{Deserialize, Serialize};

use super::bimap::BiMapIndex;
use super::{bimap::IntoBiMapIndex, ClassRef, ClassRefIdx, StringIdx, Type};
use crate::field_desc::FieldDescriptor as V1Field;
use crate::static_field_desc::StaticFieldDescriptor as V1StaticField;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct FieldIdx(BiMapIndex);
impl IntoBiMapIndex for FieldIdx {
    fn from_hash(val: BiMapIndex) -> Self {
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
    fn new(owner: ClassRefIdx, name: StringIdx, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    pub(crate) fn from_v1(desc: &V1Field, asm: &mut super::Assembly) -> Self {
        let owner = ClassRef::from_v1(desc.owner(), asm);
        let owner = asm.class_idx(owner);

        Self::new(
            owner,
            asm.alloc_string(desc.name()),
            Type::from_v1(desc.tpe(), asm),
        )
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct StaticFieldIdx(BiMapIndex);
impl IntoBiMapIndex for StaticFieldIdx {
    fn from_hash(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct StaticFieldDesc {
    owner: Option<ClassRefIdx>,
    name: StringIdx,
    tpe: Type,
}

impl StaticFieldDesc {
    fn new(owner: Option<ClassRefIdx>, name: StringIdx, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    pub(crate) fn from_v1(desc: &V1StaticField, asm: &mut super::Assembly) -> Self {
        let owner = if let Some(owner) = desc.owner() {
            let owner = ClassRef::from_v1(owner, asm);
            Some(asm.class_idx(owner))
        } else {
            None
        };

        Self::new(
            owner,
            asm.alloc_string(desc.name()),
            Type::from_v1(desc.tpe(), asm),
        )
    }
}
