use crate::{
    access_modifier::AccessModifer,
    cil_op::FieldDescriptor,
    method::Method,
    r#type::{DotnetTypeRef, Type},
    utilis::{enum_tag_size, monomorphize, tag_from_enum_variants},
    IString,
};
use rustc_middle::ty::{
    AdtDef, AdtKind, AliasKind, GenericArg, Instance, List, Ty, TyCtxt, TyKind,
};
use serde::{Deserialize, Serialize};
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
}
impl TypeDef {
    pub fn ptr_components(name: &str, metadata: Type) -> Self {
        let mut ptr_components = crate::r#type::TypeDef::nameonly(name);
        ptr_components.add_field("data_address".into(), Type::Ptr(Type::Void.into()));
        ptr_components.add_field("metadata".into(), metadata);
        ptr_components
    }
    pub fn morphic_fields<'a>(
        &'a self,
        generics: &'a [Type],
    ) -> impl Iterator<Item = Option<(&'a str, Type)>> + 'a {
        self.fields()
            .iter()
            .map(|(name, tpe)| Some((name.as_ref(), tpe.map_generic(generics)?)))
    }
    pub fn set_generic_count(&mut self, generic_count: u32) {
        self.gargc = generic_count;
    }
    /// Gets a [`FieldDescriptor`] describing to a rust filed at rust field index `rust_field_idx`.
    pub fn field_desc_from_rust_field_idx(
        &self,
        self_tpe_ref: DotnetTypeRef,
        rust_field_idx: u32,
    ) -> FieldDescriptor {
        let mut field_iter = self.fields.iter();
        // If explicit offsets present, check for enum tags
        if let Some(offsets) = &self.explicit_offsets {
            if offsets[0] == 0 && self.fields()[0].0.as_ref() == "_tag" {
                field_iter.next();
            }
        };
        // Get the nth field
        let (field_name, field_type) = field_iter
            .nth(rust_field_idx as usize)
            .expect("`field_desc_from_rust_field_idx` could not find field info!");
        FieldDescriptor::new(self_tpe_ref, field_type.clone(), field_name.clone())
    }
    pub fn gargc(&self) -> u32 {
        self.gargc
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn access_modifier(&self) -> AccessModifer {
        self.access
    }
    pub fn extends(&self) -> Option<&DotnetTypeRef> {
        self.extends.as_ref()
    }
    pub fn fields(&self) -> &[(IString, Type)] {
        &self.fields
    }
    pub fn add_field(&mut self, name: IString, tpe: Type) {
        self.fields.push((name, tpe));
    }
    pub fn inner_types(&self) -> &[Self] {
        &self.inner_types
    }
    pub fn explicit_offsets(&self) -> Option<&Vec<u32>> {
        self.explicit_offsets.as_ref()
    }
    pub fn add_method(&mut self, method: Method) {
        self.functions.push(method);
    }
    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.functions.iter()
    }
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
        }
    }
    pub fn new(
        access: AccessModifer,
        name: IString,
        inner_types: Vec<Self>,
        fields: Vec<(IString, Type)>,
        functions: Vec<Method>,
        explicit_offsets: Option<Vec<u32>>,
        gargc: u32,
        extends: Option<DotnetTypeRef>,
    ) -> Self {
        Self {
            access,
            name,
            inner_types,
            fields,
            functions,
            explicit_offsets,
            gargc,
            extends,
        }
    }
}
impl From<TypeDef> for Type {
    fn from(val: TypeDef) -> Type {
        Type::DotnetType(DotnetTypeRef::new(None, val.name()).into())
    }
}
impl From<&TypeDef> for Type {
    fn from(val: &TypeDef) -> Type {
        Type::DotnetType(DotnetTypeRef::new(None, val.name()).into())
    }
}
impl From<TypeDef> for DotnetTypeRef {
    fn from(val: TypeDef) -> DotnetTypeRef {
        DotnetTypeRef::new(None, val.name())
    }
}
impl From<&TypeDef> for DotnetTypeRef {
    fn from(val: &TypeDef) -> DotnetTypeRef {
        DotnetTypeRef::new(None, val.name())
    }
}
pub fn escape_field_name(name: &str) -> IString {
    if name.is_empty() {
        "fld".into()
    } else if !name.chars().next().unwrap().is_alphabetic()
        || name == "value"
        || name == "flags"
        || name == "alignment"
        || name == "init"
    {
        format!("m_{name}").into()
    } else {
        name.into()
    }
}
pub fn ident_gargs(gargc: usize) -> std::borrow::Cow<'static, [Type]> {
    const ZERO_GARGS: &[Type] = &[];
    const ONE_GARG: &[Type] = &[Type::GenericArg(0)];
    match gargc {
        0 => ZERO_GARGS.into(),
        1 => ONE_GARG.into(),
        _ => (0..gargc)
            .map(|g| Type::GenericArg(g as u32))
            .collect::<Vec<_>>()
            .into(),
    }
}
pub fn get_array_type(element_count: usize) -> TypeDef {
    use crate::cil_op::{CILOp, FieldDescriptor};
    let name = format!("Arr{element_count}");
    let mut fields = Vec::with_capacity(element_count);
    for field in 0..element_count {
        fields.push((format!("f_{field}").into(), Type::GenericArg(0)))
    }
    let mut def = TypeDef {
        access: AccessModifer::Public,
        name: name.into(),
        inner_types: vec![],
        fields,
        functions: vec![],
        explicit_offsets: None,
        gargc: 1,
        extends: None,
    };
    // set_Item(usize offset, G0 value)
    let mut set_usize = Method::new(
        AccessModifer::Public,
        false,
        crate::function_sig::FnSig::new(
            &[(&def).into(), Type::USize, Type::GenericArg(0)],
            &Type::Void,
        ),
        "set_Item",
        vec![],
    );
    let ops = vec![
        CILOp::LDArg(0),
        CILOp::LDFieldAdress(FieldDescriptor::boxed(
            (&def).into(),
            Type::GenericArg(0),
            "f_0".to_string().into(),
        )),
        CILOp::LDArg(1),
        CILOp::Add,
        CILOp::LDArg(2),
        CILOp::STObj(Type::GenericArg(0).into()),
        CILOp::Ret,
    ];
    set_usize.set_ops(ops);
    def.add_method(set_usize);
    // get_Address(usize offset)
    let mut get_adress_usize = Method::new(
        AccessModifer::Public,
        false,
        crate::function_sig::FnSig::new(
            &[(&def).into(), Type::USize],
            &Type::Ptr(Type::GenericArg(0).into()),
        ),
        "get_Address",
        vec![],
    );
    let ops = vec![
        CILOp::LDArg(0),
        CILOp::LDFieldAdress(FieldDescriptor::boxed(
            (&def).into(),
            Type::GenericArg(0),
            "f_0".to_string().into(),
        )),
        CILOp::LDArg(1),
        CILOp::Add,
        CILOp::Ret,
    ];
    get_adress_usize.set_ops(ops);
    def.add_method(get_adress_usize);
    // get_Item
    let mut get_item_usize = Method::new(
        AccessModifer::Public,
        false,
        crate::function_sig::FnSig::new(&[(&def).into(), Type::USize], &Type::GenericArg(0)),
        "get_Item",
        vec![],
    );
    let ops = vec![
        CILOp::LDArg(0),
        CILOp::LDFieldAdress(FieldDescriptor::boxed(
            (&def).into(),
            Type::GenericArg(0),
            "f_0".to_string().into(),
        )),
        CILOp::LDArg(1),
        CILOp::Add,
        CILOp::LdObj(Type::GenericArg(0).into()),
        CILOp::Ret,
    ];
    get_item_usize.set_ops(ops);
    def.add_method(get_item_usize);
    def
}
