use serde::{Deserialize, Serialize};

use crate::{access_modifier::AccessModifer, method::Method, DotnetTypeRef, IString, Type};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TypeDef {
    access: AccessModifer,
    name: IString,
    inner_types: Vec<Self>,
    fields: Vec<(IString, Type)>,
    functions: Vec<Method>,
    explicit_offsets: Option<Vec<u32>>,
    gargc: u32,
    extends: Option<DotnetTypeRef>,
    explict_size: Option<u64>,
    //requires_aligement_adjustements:bool,
}
impl TypeDef {
    pub fn set_generic_count(&mut self, generic_count: u32) {
        self.gargc = generic_count;
    }

    fn field_types(&self) -> impl Iterator<Item = &Type> {
        self.fields().iter().map(|(_, tpe)| tpe)
    }
    pub fn all_types(&self) -> impl Iterator<Item = &Type> {
        //TODO: this breaks if a type contains more than one layer of nested types!
        self.field_types()
            .chain(self.inner_types().iter().flat_map(TypeDef::field_types))
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
    pub fn extends(&self) -> Option<&DotnetTypeRef> {
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
            explict_size: None,
        }
    }
    #[must_use]
    pub fn new(
        access: AccessModifer,
        name: IString,
        inner_types: Vec<Self>,
        fields: Vec<(IString, Type)>,
        functions: Vec<Method>,
        explicit_offsets: Option<Vec<u32>>,
        gargc: u32,
        extends: Option<DotnetTypeRef>,
        explict_size: Option<u64>,
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
    pub fn explict_size(&self) -> Option<u64> {
        self.explict_size
    }

    fn sanity_check(&self) {
        if let Some(size) = self.explict_size() {
            self.explicit_offsets().iter().flat_map(|vec|*vec).for_each(|offset|assert!(*offset <= u32::try_from(size).unwrap(), "Sanity check failed! The size of type {name} is {size}, yet it has a filed at offset {offset}",name = self.name));
        }
        if let Some(offsets) = self.explicit_offsets() {
            assert_eq!(
                offsets.len(),
                self.fields().len(),
                "Sanity check failed! Type {name} has a field decl / field offset mismatch.",
                name = self.name()
            );
        }
    }
}
impl From<TypeDef> for Type {
    fn from(val: TypeDef) -> Type {
        Type::DotnetType(DotnetTypeRef::new::<&str, _>(None, val.name()).into())
    }
}
impl From<&TypeDef> for Type {
    fn from(val: &TypeDef) -> Type {
        Type::DotnetType(DotnetTypeRef::new::<&str, _>(None, val.name()).into())
    }
}
impl From<TypeDef> for DotnetTypeRef {
    fn from(val: TypeDef) -> DotnetTypeRef {
        DotnetTypeRef::new::<&str, _>(None, val.name())
    }
}
impl From<&TypeDef> for DotnetTypeRef {
    fn from(val: &TypeDef) -> DotnetTypeRef {
        DotnetTypeRef::new::<&str, _>(None, val.name())
    }
}
