use serde::{Deserialize, Serialize};

use crate::DotnetTypeRef as V1ClassRef;

use super::{bimap::HashWrapper, StringIdx, Type};

impl From<ClassIdx> for Type {
    fn from(val: ClassIdx) -> Self {
        Type::ClassRef(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug, Copy, Serialize, Deserialize)]
pub struct ClassIdx(u64);
impl HashWrapper for ClassIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ClassRef {
    name: StringIdx,
    asm: Option<StringIdx>,
    is_valuetype: bool,
    generics: Box<[Type]>,
}

impl ClassRef {
    pub fn new(
        name: StringIdx,
        asm: Option<StringIdx>,
        is_valuetype: bool,
        generics: Box<[Type]>,
    ) -> Self {
        Self {
            name,
            asm,
            is_valuetype,
            generics,
        }
    }

    pub(crate) fn from_v1(dotnet_type: &V1ClassRef, asm: &mut super::Assembly) -> ClassRef {
        match dotnet_type {
            V1ClassRef::Full {
                assembly,
                name_path,
                generics,
                is_valuetype,
            } => {
                let name = asm.alloc_string(name_path.clone());
                let assembly = assembly.clone().map(|assembly| asm.alloc_string(assembly));
                let generics: Box<[_]> =
                    generics.iter().map(|gen| Type::from_v1(gen, asm)).collect();
                ClassRef::new(name, assembly, *is_valuetype, generics)
            }
            V1ClassRef::OptimizedRustStruct { name: _ } => panic!(),
        }
    }
}
