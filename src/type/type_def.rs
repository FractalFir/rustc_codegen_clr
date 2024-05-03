use crate::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    cil::{CallSite, FieldDescriptor},
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    conv_usize, ld_field_address,
    method::{Method, MethodType},
    r#type::{DotnetTypeRef, Type},
    size_of,
    utilis::adt::FieldOffsetIterator,
    IString,
};
use rustc_span::def_id::DefId;
use rustc_target::abi::Layout;
use serde::{Deserialize, Serialize};
pub(crate) const CUSTOM_INTEROP_TYPE_DEF: &str = "RustcCLRInteropManagedCustomTypeDef";
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
}
impl TypeDef {
    #[must_use]
    pub fn ptr_components(name: &str, metadata: Type) -> Self {
        let mut ptr_components = crate::r#type::TypeDef::nameonly(name);
        ptr_components.add_field("data_pointer".into(), Type::Ptr(Type::Void.into()));
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

    fn field_types(&self) -> impl Iterator<Item = &Type> {
        self.fields().iter().map(|(_, tpe)| tpe)
    }
    pub fn all_types(&self) -> impl Iterator<Item = &Type> {
        //TODO: this breaks if a type contains more than one layer of nested types!
        self.field_types().chain(
            self.inner_types()
                .iter()
                .flat_map(|sub_tpe| sub_tpe.field_types()),
        )
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

    pub fn explict_size(&self) -> Option<u64> {
        self.explict_size
    }

    fn sanity_check(&self) {
        if let Some(size) = self.explict_size() {
            self.explicit_offsets().iter().flat_map(|vec|*vec).for_each(|offset|if *offset > size as u32{
                panic!("Sanity check failed! The size of type {name} is {size}, yet it has a filed at offset {offset}",name = self.name);
            });
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
#[must_use]
pub fn escape_field_name(name: &str) -> IString {
    match name.chars().next() {
        None => "fld".into(),
        Some(first) => {
            if !(first.is_alphabetic() || first == '_')
        || name == "value"
        || name == "flags"
        || name == "alignment"
        || name == "init"
        || name == "string"
        || name == "nint"
        || name == "nuint"
        || name == "out"
        || name == "rem"
        || name == "add"
        || name == "div"
        || name == "error"
        || name == "opt"
        || name == "private"
        || name == "public"
        || name == "object"
        || name == "class"
        //FIXME: this is a sign of a bug. ALL fields not starting with a letter should have been caught by the statement above.
        || name == "0"
            {
                format!("m_{name}").into()
            } else {
                name.into()
            }
        }
    }
}
pub fn closure_name(_def_id: DefId, fields: &[Type], _sig: &crate::function_sig::FnSig) -> String {
    let mangled_fields: String = fields.iter().map(crate::r#type::mangle).collect();
    format!(
        "Closure{field_count}{mangled_fields}",
        field_count = fields.len()
    )
}
pub fn closure_typedef(
    def_id: DefId,
    fields: &[Type],
    sig: crate::function_sig::FnSig,
    layout: &Layout,
) -> TypeDef {
    let name = closure_name(def_id, fields, &sig);
    let fields: Vec<_> = fields
        .iter()
        .enumerate()
        .map(|(idx, ty)| (format!("f_{idx}").into(), ty.clone()))
        .collect();
    let offsets: Vec<_> = FieldOffsetIterator::fields(layout).collect();
    assert_eq!(fields.len(), offsets.len());
    TypeDef::new(
        AccessModifer::Public,
        name.into(),
        vec![],
        fields,
        vec![],
        Some(offsets),
        0,
        None,
        Some(layout.size().bytes()),
    )
}
pub fn arr_name(element_count: usize, element: &Type) -> IString {
    let element_name = super::mangle(element);
    format!("Arr{element_count}_{element_name}",).into()
}
pub fn tuple_name(elements: &[Type]) -> IString {
    let generics: String = elements.iter().map(super::mangle).collect();
    format!(
        "Tuple{generic_count}{generics}",
        generic_count = generics.len()
    )
    .into()
}

#[must_use]
pub fn tuple_typedef(elements: &[Type], layout: &Layout) -> TypeDef {
    let name = tuple_name(elements);
    let fields: Vec<_> = elements
        .iter()
        .enumerate()
        .map(|(idx, ele)| (format!("Item{}", idx + 1).into(), ele.clone()))
        .collect();
    let explicit_offsets = FieldOffsetIterator::fields(layout).collect();
    TypeDef::new(
        AccessModifer::Public,
        name,
        vec![],
        fields,
        vec![],
        Some(explicit_offsets),
        0,
        None,
        None,
    )
}
#[must_use]
pub fn get_array_type(element_count: usize, element: Type, explict_size: u64) -> TypeDef {
    let name = arr_name(element_count, &element);
    let mut fields = Vec::with_capacity(element_count);
    let element_size = if explict_size != 0 {
        assert!(
            explict_size % element_count as u64 == 0,
            "The total array size must be divisible by its element count."
        );
        explict_size / (element_count as u64)
    } else {
        // WARNING: ZSTs in .NET aren't real(they have size of 1). Handle zero-sized arrays with caution!
        0
    };

    let mut explicit_offsets = Vec::with_capacity(element_count);
    for field in 0..element_count {
        fields.push((format!("f_{field}").into(), element.clone()));
        explicit_offsets.push((field as u64 * element_size) as u32);
    }
    let mut def = TypeDef {
        access: AccessModifer::Public,
        name,
        inner_types: vec![],
        fields,
        functions: vec![],
        explicit_offsets: Some(explicit_offsets),
        gargc: 0,
        extends: None,
        explict_size: Some(explict_size),
    };
    let _as_pointer = CallSite::ref_as_ptr(element.clone());
    // set_Item(usize offset, G0 value)
    if element_count > 0 {
        let set_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            crate::function_sig::FnSig::new(
                &[
                    Type::Ptr(Box::new(def.clone().into())),
                    Type::USize,
                    element.clone(),
                ],
                &Type::Void,
            ),
            "set_Item",
            vec![],
            vec![BasicBlock::new(
                vec![
                    CILRoot::STObj {
                        tpe: element.clone().into(),
                        addr_calc: conv_usize!(ld_field_address!(
                            CILNode::LDArg(0),
                            FieldDescriptor::boxed(
                                (&def).into(),
                                element.clone(),
                                "f_0".to_string().into(),
                            )
                        )) + CILNode::LDArg(1) * size_of!(element.clone()),
                        value_calc: CILNode::LDArg(2),
                    }
                    .into(),
                    CILRoot::VoidRet.into(),
                ],
                0,
                None,
            )],
        );
        def.add_method(set_usize);

        // get_Address(usize offset)
        let get_adress_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            crate::function_sig::FnSig::new(
                &[Type::Ptr(Box::new(def.clone().into())), Type::USize],
                &Type::Ptr(element.clone().into()),
            ),
            "get_Address",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: conv_usize!(ld_field_address!(
                        CILNode::LDArg(0),
                        FieldDescriptor::boxed(
                            (&def).into(),
                            element.clone(),
                            "f_0".to_string().into(),
                        )
                    )) + CILNode::LDArg(1) * size_of!(element.clone()),
                }
                .into()],
                0,
                None,
            )],
        );
        def.add_method(get_adress_usize);
        // get_Item
        let get_item_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            crate::function_sig::FnSig::new(
                &[Type::Ptr(Box::new(def.clone().into())), Type::USize],
                &element.clone(),
            ),
            "get_Item",
            vec![],
            vec![BasicBlock::new(
                vec![CILRoot::Ret {
                    tree: CILNode::LdObj {
                        ptr: Box::new(
                            conv_usize!(ld_field_address!(
                                CILNode::LDArg(0),
                                FieldDescriptor::boxed(
                                    (&def).into(),
                                    element.clone(),
                                    "f_0".to_string().into(),
                                )
                            )) + CILNode::LDArg(1) * size_of!(element.clone()),
                        ),
                        obj: Box::new(element),
                    },
                }
                .into()],
                0,
                None,
            )],
        );

        def.add_method(get_item_usize);

        //to_string.set_ops(ops);
        //def.add_method(to_string);
    }
    def
}
