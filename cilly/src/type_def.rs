use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::{access_modifier::AccessModifer, method::Method, ClassRef, IString, Type};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct ClassDef {
    access: AccessModifer,
    name: IString,
    inner_types: Vec<Self>,
    fields: Vec<(IString, Type)>,
    functions: Vec<Method>,
    explicit_offsets: Option<Vec<u32>>,
    gargc: u32,
    extends: Option<ClassRef>,
    explict_size: Option<NonZeroU32>,
    //requires_aligement_adjustements:bool,
}
impl ClassDef {
    pub fn set_generic_count(&mut self, generic_count: u32) {
        self.gargc = generic_count;
    }

    fn field_types(&self) -> impl Iterator<Item = &Type> {
        self.fields().iter().map(|(_, tpe)| tpe)
    }
    pub fn all_types(&self) -> impl Iterator<Item = &Type> {
        //TODO: this breaks if a type contains more than one layer of nested types!
        self.field_types()
            .chain(self.inner_types().iter().flat_map(ClassDef::field_types))
    }
    #[must_use]
    pub fn gargc(&self) -> u32 {
        self.gargc
    }
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    #[must_use]
    pub fn access_modifier(&self) -> AccessModifer {
        self.access
    }
    #[must_use]
    pub fn extends(&self) -> Option<&ClassRef> {
        self.extends.as_ref()
    }
    #[must_use]
    pub fn fields(&self) -> &[(IString, Type)] {
        &self.fields
    }
    pub fn add_field(&mut self, name: IString, tpe: Type) {
        self.fields.push((name, tpe));
    }
    #[must_use]
    pub fn inner_types(&self) -> &[Self] {
        &self.inner_types
    }
    #[must_use]
    pub fn explicit_offsets(&self) -> Option<&Vec<u32>> {
        self.explicit_offsets.as_ref()
    }
    pub fn add_method(&mut self, method: Method) {
        self.functions.push(method);
    }
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.iter()
    }
    pub fn methods_mut(&mut self) -> impl Iterator<Item = &mut Method> {
        self.functions.iter_mut()
    }
    #[must_use]
    pub fn nameonly(name: &str) -> Self {
        Self {
            access: AccessModifer::Public,
            name: name.into(),
            inner_types: vec![],
            fields: vec![],
            functions: vec![],
            gargc: 0,
            extends: None,
            explicit_offsets: None,
            explict_size: Some(NonZeroU32::new(1).unwrap()),
        }
    }
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        access: AccessModifer,
        name: IString,
        inner_types: Vec<Self>,
        fields: Vec<(IString, Type)>,
        functions: Vec<Method>,
        explicit_offsets: Option<Vec<u32>>,
        gargc: u32,
        extends: Option<ClassRef>,
        explict_size: Option<NonZeroU32>,
    ) -> Self {
        let res = Self {
            access,
            name,
            inner_types,
            fields,
            functions,
            explicit_offsets,
            gargc,
            extends,
            explict_size,
        };
        //TODO:consider having this enabled only for debug
        res.sanity_check();
        res
    }

    #[must_use]
    pub const fn explict_size(&self) -> Option<NonZeroU32> {
        self.explict_size
    }

    fn sanity_check(&self) {
        if let Some(size) = self.explict_size() {
            self.explicit_offsets().iter().flat_map(|vec|*vec).for_each(|offset|assert!(*offset <= size.get(), "Sanity check failed! The size of type {name} is {size}, yet it has a filed at offset {offset}",name = self.name));
        }
        if let Some(offsets) = self.explicit_offsets() {
            assert_eq!(
                offsets.len(),
                self.fields().len(),
                "Sanity check failed! Type {name} has a field decl / field offset mismatch.",
                name = self.name()
            );
            let max_offset = offsets.iter().max().unwrap_or(&0);
            let explict_size = self
                .explict_size()
                .unwrap_or_else(|| {
                    panic!(
                        "Explict offsets provided without explicit size. Type: {}",
                        self.name()
                    )
                })
                .get();
            assert!(
                (*max_offset) < explict_size,
                "name:{:?} max_offset:{max_offset} explict_size:{explict_size} offsets:{:?} fields:{:?}",
                self.name(),
                self.explicit_offsets().unwrap(),
                self.fields()
            );
        }
        self.field_types()
            .for_each(|tpe| assert_ne!(*tpe, Type::Void));
    }
}
impl From<ClassDef> for Type {
    fn from(val: ClassDef) -> Type {
        Type::ClassRef(ClassRef::new::<&str, _>(None, val.name()).into())
    }
}
impl From<&ClassDef> for Type {
    fn from(val: &ClassDef) -> Type {
        Type::ClassRef(ClassRef::new::<&str, _>(None, val.name()).into())
    }
}
impl From<ClassDef> for ClassRef {
    fn from(val: ClassDef) -> ClassRef {
        ClassRef::new::<&str, _>(None, val.name())
    }
}
impl From<&ClassDef> for ClassRef {
    fn from(val: &ClassDef) -> ClassRef {
        ClassRef::new::<&str, _>(None, val.name())
    }
}
