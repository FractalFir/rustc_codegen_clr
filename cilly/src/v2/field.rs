use serde::{Deserialize, Serialize};

use crate::IString;

use super::bimap::{BiMapIndex, Interned};
use super::{bimap::IntoBiMapIndex, Type};
use super::{ClassRef, Int, IntoAsmIndex};

impl Interned<FieldDesc> {
    pub fn data_ptr(asm: &mut super::Assembly, res: Interned<ClassRef>) -> Interned<FieldDesc> {
        let name = asm.alloc_string(crate::DATA_PTR);
        let tpe = asm.nptr(Type::Void);
        asm.alloc_field(FieldDesc {
            owner: res,
            name,
            tpe,
        })
    }

    pub fn metadata(asm: &mut super::Assembly, res: Interned<ClassRef>) -> Interned<FieldDesc> {
        let name = asm.alloc_string(crate::METADATA);
        asm.alloc_field(FieldDesc {
            owner: res,
            name,
            tpe: Type::Int(Int::USize),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct FieldDesc {
    owner: Interned<ClassRef>,
    name: Interned<IString>,
    tpe: Type,
}

impl FieldDesc {
    #[must_use]
    pub fn new(owner: Interned<ClassRef>, name: Interned<IString>, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    #[must_use]
    pub fn owner(&self) -> Interned<ClassRef> {
        self.owner
    }

    #[must_use]
    pub fn name(&self) -> Interned<IString> {
        self.name
    }

    #[must_use]
    pub fn tpe(&self) -> Type {
        self.tpe
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct StaticFieldDesc {
    owner: Interned<ClassRef>,
    name: Interned<IString>,
    tpe: Type,
}

impl StaticFieldDesc {
    #[must_use]
    pub fn new(owner: Interned<ClassRef>, name: Interned<IString>, tpe: Type) -> Self {
        Self { owner, name, tpe }
    }

    #[must_use]
    pub fn owner(&self) -> Interned<ClassRef> {
        self.owner
    }

    #[must_use]
    pub fn name(&self) -> Interned<IString> {
        self.name
    }

    #[must_use]
    pub fn tpe(&self) -> Type {
        self.tpe
    }
}
impl IntoAsmIndex<Interned<FieldDesc>> for FieldDesc {
    fn into_idx(self, asm: &mut super::Assembly) -> Interned<FieldDesc> {
        asm.alloc_field(self)
    }
}
