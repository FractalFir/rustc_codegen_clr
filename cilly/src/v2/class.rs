use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::{v2::MethodDef, DotnetTypeRef as V1ClassRef};

use super::{access::Access, bimap::HashWrapper, MethodDefIdx, StringIdx, Type};

impl From<ClassRefIdx> for Type {
    fn from(val: ClassRefIdx) -> Self {
        Type::ClassRef(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug, Copy, Serialize, Deserialize)]
pub struct ClassRefIdx(u64);
impl HashWrapper for ClassRefIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
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
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct ClassDef {
    name: StringIdx,
    is_valuetype: bool,
    generics: u32,
    extends: Option<ClassRefIdx>,
    fields: Vec<(Type, StringIdx, Option<u32>)>,
    static_fields: Vec<(Type, StringIdx, bool)>,
    methods: Vec<MethodDefIdx>,
    access: Access,
    explict_size: Option<NonZeroU32>,
}

impl ClassDef {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: StringIdx,
        is_valuetype: bool,
        generics: u32,
        extends: Option<ClassRefIdx>,
        fields: Vec<(Type, StringIdx, Option<u32>)>,
        static_fields: Vec<(Type, StringIdx, bool)>,
        methods: Vec<MethodDefIdx>,
        access: Access,
        explict_size: Option<NonZeroU32>,
    ) -> Self {
        Self {
            name,
            is_valuetype,
            generics,
            extends,
            fields,
            static_fields,
            methods,
            access,
            explict_size,
        }
    }

    pub(crate) fn ref_to(&self) -> ClassRef {
        assert_eq!(self.generics, 0);
        ClassRef::new(self.name, None, false, vec![].into())
    }

    pub fn methods_mut(&mut self) -> &mut Vec<MethodDefIdx> {
        &mut self.methods
    }

    pub fn static_fields_mut(&mut self) -> &mut Vec<(Type, StringIdx, bool)> {
        &mut self.static_fields
    }

    pub(crate) fn from_v1(
        tdef: &crate::type_def::TypeDef,
        asm: &mut super::Assembly,
    ) -> ClassDefIdx {
        let fields: Vec<_> = if let Some(offsets) = tdef.explicit_offsets() {
            tdef.fields()
                .iter()
                .zip(offsets)
                .map(|((name, tpe), offset)| {
                    let name = asm.alloc_string(name.clone());
                    let tpe = Type::from_v1(tpe, asm);

                    (tpe, name, Some(*offset))
                })
                .collect()
        } else {
            tdef.fields()
                .iter()
                .map(|(name, tpe)| {
                    let name = asm.alloc_string(name.clone());
                    let tpe = Type::from_v1(tpe, asm);

                    (tpe, name, None)
                })
                .collect()
        };
        assert!(tdef.inner_types().is_empty());
        let name = asm.alloc_string(tdef.name());
        let generics = tdef.gargc();
        let extends = if let Some(extends) = tdef.extends() {
            let cref = ClassRef::from_v1(extends, asm);
            Some(asm.class_idx(cref))
        } else {
            None
        };
        let def = Self::new(
            name,
            tdef.extends().is_none(),
            generics,
            extends,
            fields,
            vec![],
            vec![],
            Access::Public,
            tdef.explict_size(),
        );
        let defid = asm.class_def(def);
        let methods: Vec<_> = tdef
            .methods()
            .map(|method| {
                let def = MethodDef::from_v1(method, asm, defid);
                asm.new_method(def)
            })
            .collect();
        let def = asm.class_mut(defid);
        def.methods_mut().extend(methods);
        defid
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug, Copy, Serialize, Deserialize)]
pub struct ClassDefIdx(pub ClassRefIdx);

impl std::ops::Deref for ClassDefIdx {
    type Target = ClassRefIdx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
