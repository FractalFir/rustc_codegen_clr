use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::{v2::MethodDef, DotnetTypeRef as V1ClassRef};

use super::{
    access::Access,
    bimap::{BiMapIndex, IntoBiMapIndex},
    MethodDefIdx, StringIdx, Type,
};

impl From<ClassRefIdx> for Type {
    fn from(val: ClassRefIdx) -> Self {
        Type::ClassRef(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ClassRefIdx(BiMapIndex);
impl IntoBiMapIndex for ClassRefIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
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

    pub fn from_v1(dotnet_type: &V1ClassRef, asm: &mut super::Assembly) -> ClassRef {
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

    pub fn asm(&self) -> Option<StringIdx> {
        self.asm
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }

    pub fn generics(&self) -> &[Type] {
        &self.generics
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
        crate::utilis::assert_unique(&methods);
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
        ClassRef::new(self.name, None, self.is_valuetype, vec![].into())
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
            Some(asm.alloc_class_ref(cref))
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
        tdef.methods().for_each(|method| {
            let def = MethodDef::from_v1(method, asm, defid);
            asm.new_method(def);
        });
        defid
    }

    pub fn access(&self) -> &Access {
        &self.access
    }

    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }

    pub fn extends(&self) -> Option<ClassRefIdx> {
        self.extends
    }

    pub(crate) fn has_explicit_layout(&self) -> bool {
        self.explict_size.is_some() && self.fields.iter().any(|(_, _, offset)| offset.is_some())
    }

    pub fn fields(&self) -> &[(Type, StringIdx, Option<u32>)] {
        &self.fields
    }

    pub fn name(&self) -> StringIdx {
        self.name
    }

    pub fn static_fields(&self) -> &[(Type, StringIdx, bool)] {
        &self.static_fields
    }

    pub fn methods(&self) -> &[MethodDefIdx] {
        &self.methods
    }

    pub fn explict_size(&self) -> Option<NonZeroU32> {
        self.explict_size
    }

    pub fn generics(&self) -> u32 {
        self.generics
    }

    pub(crate) fn merge_defs(&mut self, translated: ClassDef) {
        // Check name matches
        assert_eq!(self.name(), translated.name());
        // Check valuetype matches
        assert_eq!(self.is_valuetype(), translated.is_valuetype());
        // Check generic count matches
        assert_eq!(self.generics(), translated.generics());
        // Check inheretence matches
        assert_eq!(self.extends(), translated.extends());
        // If we want to merge types, we need to confoirm they have identical fields.
        assert_eq!(self.fields(), translated.fields());
        // Merge the static fields, removing duplicates
        self.static_fields_mut().extend(translated.static_fields());
        make_unique(&mut self.static_fields);
        // Merge the static fields, removing duplicates
        self.static_fields_mut().extend(translated.static_fields());
        make_unique(self.static_fields_mut());
        // Merge the methods, removing duplicates
        self.methods_mut().extend(translated.methods());
        make_unique(self.methods_mut());
        // Check accessibility matches
        assert_eq!(self.access(), translated.access());
        // Check size matches
        assert_eq!(self.explict_size(), translated.explict_size());
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy, Serialize, Deserialize)]
pub struct ClassDefIdx(pub ClassRefIdx);

impl std::ops::Deref for ClassDefIdx {
    type Target = ClassRefIdx;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn into_unique<T: Eq + std::hash::Hash>(input: Vec<T>) -> Vec<T> {
    let set: fxhash::FxHashSet<_> = input.into_iter().collect();
    set.into_iter().collect()
}
fn make_unique<T: Eq + std::hash::Hash>(input: &mut Vec<T>) {
    let mut tmp = Vec::new();
    std::mem::swap(&mut tmp, input);
    let mut tmp = into_unique(tmp);
    std::mem::swap(&mut tmp, input);
}
