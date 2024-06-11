use crate::{utilis::adt::FieldOffsetIterator, IString};
use cilly::{
    access_modifier::AccessModifer,
    basic_block::BasicBlock,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_usize,
    field_desc::FieldDescriptor,
    ld_field_address,
    method::{Method, MethodType},
    size_of,
    type_def::TypeDef,
    Type,
};
use rustc_span::def_id::DefId;
use rustc_target::abi::Layout;
use std::num::NonZeroU64;

pub(crate) const CUSTOM_INTEROP_TYPE_DEF: &str = "RustcCLRInteropManagedCustomTypeDef";

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
pub fn closure_name(_def_id: DefId, fields: &[Type], _sig: &cilly::fn_sig::FnSig) -> String {
    let mangled_fields: String = fields.iter().map(crate::r#type::mangle).collect();
    format!(
        "Closure{field_count}{mangled_fields}",
        field_count = fields.len()
    )
}
#[must_use]
pub fn closure_typedef(
    def_id: DefId,
    fields: &[Type],
    sig: &cilly::fn_sig::FnSig,
    layout: Layout,
) -> TypeDef {
    let name = closure_name(def_id, fields, sig);
    let field_iter = fields
        .iter()
        .enumerate()
        .map(|(idx, ty)| (format!("f_{idx}").into(), ty.clone()));

    let offset_iter = FieldOffsetIterator::fields((*layout.0).clone());
    let mut explicit_offsets = Vec::new();
    let mut fields = Vec::new();
    for ((name, field), offset) in (field_iter).zip(offset_iter) {
        if field == Type::Void {
            continue;
        }
        fields.push((name, field));
        explicit_offsets.push(offset);
    }
    assert_eq!(fields.len(), explicit_offsets.len());
    TypeDef::new(
        AccessModifer::Public,
        name.into(),
        vec![],
        fields,
        vec![],
        Some(explicit_offsets),
        0,
        None,
        Some(NonZeroU64::new(layout.size().bytes()).unwrap()),
    )
}
#[must_use]
pub fn arr_name(element_count: usize, element: &Type) -> IString {
    cilly::arr_name(element_count, element)
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
pub fn tuple_typedef(elements: &[Type], layout: Layout) -> TypeDef {
    let name = tuple_name(elements);
    let field_iter = elements
        .iter()
        .enumerate()
        .map(|(idx, ele)| (format!("Item{}", idx + 1).into(), ele.clone()));
    let explicit_offset_iter = FieldOffsetIterator::fields((*layout.0).clone());
    let mut explicit_offsets = Vec::new();
    let mut fields = Vec::new();
    for ((name, field), offset) in (field_iter).zip(explicit_offset_iter) {
        if field == Type::Void {
            continue;
        }
        fields.push((name, field));
        explicit_offsets.push(offset);
    }

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
        explicit_offsets.push(u32::try_from(field as u64 * element_size).unwrap());
    }
    //TODO:check array aligement
    let mut def = TypeDef::new(
        AccessModifer::Public,
        name,
        vec![],
        fields,
        vec![],
        Some(explicit_offsets),
        0,
        None,
        Some(NonZeroU64::new(explict_size).unwrap()),
    );
    if element_count > 0 {
        let set_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            cilly::fn_sig::FnSig::new(
                &[
                    Type::Ptr(Box::new(def.clone().into())),
                    Type::USize,
                    element.clone(),
                ],
                Type::Void,
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
            vec![Some("this".into()), Some("idx".into()), Some("val".into())],
        );
        def.add_method(set_usize);

        // get_Address(usize offset)
        let get_adress_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            cilly::fn_sig::FnSig::new(
                &[Type::Ptr(Box::new(def.clone().into())), Type::USize],
                Type::Ptr(element.clone().into()),
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
            vec![Some("this".into()), Some("idx".into())],
        );
        def.add_method(get_adress_usize);
        // get_Item
        let get_item_usize = Method::new(
            AccessModifer::Public,
            MethodType::Instance,
            cilly::fn_sig::FnSig::new(
                &[Type::Ptr(Box::new(def.clone().into())), Type::USize],
                element.clone(),
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
            vec![Some("this".into()), Some("idx".into())],
        );

        def.add_method(get_item_usize);

        //to_string.set_ops(ops);
        //def.add_method(to_string);
    }
    def
}
