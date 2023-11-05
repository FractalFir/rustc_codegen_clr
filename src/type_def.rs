use crate::{
    access_modifier::AccessModifer,
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
    pub fn morphic_fields<'a>(
        &'a self,
        generics: &'a [Type],
    ) -> impl Iterator<Item = (&'a str, Type)> + 'a {
        self.fields()
            .iter()
            .map(|(name, tpe)| (name.as_ref(), tpe.map_generic(generics)))
    }
    pub fn set_generic_count(&mut self, generic_count: u32) {
        self.gargc = generic_count;
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
    pub fn from_ty<'tyctx>(
        ty: Ty<'tyctx>,
        ctx: TyCtxt<'tyctx>,
        method: &Instance<'tyctx>,
    ) -> Vec<Self> {
        match ty.kind() {
            TyKind::Adt(adt_def, subst) => {
                let _name = crate::utilis::adt_name(adt_def);
                let _gargc = subst.len() as u32;
                let _access = AccessModifer::Public;
                match adt_def.adt_kind() {
                    AdtKind::Struct => Self::struct_from_adt(ty, adt_def, subst, ctx, method),
                    AdtKind::Enum => Self::enum_from_adt(ty, adt_def, subst, ctx, method),
                    AdtKind::Union => Self::union_from_adt(ty, adt_def, subst, ctx, method),
                }
            }
            TyKind::Ref(_region, inner, _mut) => Self::from_ty(*inner, ctx, method),
            TyKind::RawPtr(inner_and_mut) => Self::from_ty(inner_and_mut.ty, ctx, method),
            TyKind::Slice(inner) => Self::from_ty(*inner, ctx, method),
            TyKind::Array(element, size) => {
                let length = crate::utilis::try_resolve_const_size(size)
                    .expect("Could not resolve array size!");
                let mut types = Self::from_ty(*element, ctx, method);
                types.push(get_array_type(length));
                types
            }
            TyKind::Alias(_, alias_ty) => {
                let alias_ty = ctx.type_of(alias_ty.def_id).instantiate_identity();
                Self::from_ty(alias_ty, ctx, method)
            }
            _ => vec![],
        }
    }
    fn struct_from_adt<'tyctx>(
        original: Ty<'tyctx>,
        adt_def: &AdtDef<'tyctx>,
        subst: &'tyctx List<GenericArg<'tyctx>>,
        ctx: TyCtxt<'tyctx>,
        method: &Instance<'tyctx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        let mut gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            let resolved_field_ty = monomorphize(method, resolved_field_ty, ctx);
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx, method));
            }
        });
        for field in adt_def.all_fields() {
            //rustc_middle::ty::List::empty()
            let generic_ty = ctx.type_of(field.did).instantiate_identity();
            let ty = if let TyKind::Alias(ak, _) = generic_ty.kind() {
                assert_eq!(
                    *ak,
                    AliasKind::Projection,
                    "ERROR alias kind is not supported in adt def!"
                );
                // Increase generic count
                let curr_gargc = gargc;
                println!("Generic typedef is now supported!");
                gargc += 1;
                Type::GenericArg(curr_gargc)
            } else {
                Type::generic_from_ty(generic_ty, ctx)
            };
            let name = escape_field_name(&field.name.to_string());
            fields.push((name, ty));
        }
        res.push(Self {
            access,
            name,
            inner_types: vec![],
            fields,
            functions: vec![],
            gargc,
            extends: None,
            explicit_offsets: None,
        });
        res
    }
    fn union_from_adt<'tyctx>(
        original: Ty<'tyctx>,
        adt_def: &AdtDef<'tyctx>,
        subst: &'tyctx List<GenericArg<'tyctx>>,
        ctx: TyCtxt<'tyctx>,
        method: &Instance<'tyctx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            let resolved_field_ty = monomorphize(method, resolved_field_ty, ctx);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx, method));
            }
        });
        for field in adt_def.all_fields() {
            //rustc_middle::ty::List::empty()
            // HERE ALL GOES TO SHIT WITH MORPHIZATION.
            let generic_ty = ctx.type_of(field.did).instantiate_identity();
            if let TyKind::Alias(_, _) = generic_ty.kind() {
                panic!("UNHANDLED ERROR: type contains an associated generic!");
            }
            let ty = Type::generic_from_ty(generic_ty, ctx);
            let name = escape_field_name(&field.name.to_string());
            fields.push((name, ty));
        }
        let explicit_offsets = Some(fields.iter().map(|_| 0).collect());
        res.push(Self {
            access,
            name,
            inner_types: vec![],
            fields,
            functions: vec![],
            gargc,
            extends: None,
            explicit_offsets,
        });
        res
    }
    fn enum_from_adt<'tyctx>(
        original: Ty<'tyctx>,
        adt_def: &AdtDef<'tyctx>,
        subst: &'tyctx List<GenericArg<'tyctx>>,
        ctx: TyCtxt<'tyctx>,
        method: &Instance<'tyctx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        // Handle  `Never` type alias
        if name.to_string() == "std.convert.Infallible" {
            return vec![];
        }

        let mut gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        //let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx, method));
            }
        });
        let mut fields = vec![(
            "_tag".into(),
            tag_from_enum_variants(adt_def.variants().len() as u64),
        )];
        let mut explicit_offsets = vec![0];
        let tag_size = enum_tag_size(adt_def.variants().len() as u64);
        assert_ne!(tag_size, 0, "ERROR:{name} has a zero sized tag!");
        explicit_offsets.extend(adt_def.variants().iter().map(|_| tag_size));
        let mut inner_types = vec![];
        for variant in adt_def.variants() {
            //println!("Variant:{variant:?}");
            let variant_name = variant.name.to_string();
            let mut variant_type = DotnetTypeRef::new(None, &format!("{name}/{variant_name}"));
            variant_type.set_generics(ident_gargs(gargc as usize));
            fields.push((
                format!("v_{}", escape_field_name(variant_name.as_ref())).into(),
                Type::DotnetType(Box::new(variant_type)),
            ));
            let mut fields = vec![];
            for field in &variant.fields {
                let generic_ty = ctx.type_of(field.did).instantiate_identity();
                let ty = if let TyKind::Alias(ak, _) = generic_ty.kind() {
                    assert_eq!(
                        *ak,
                        AliasKind::Projection,
                        "ERROR alias kind is not supported in adt def!"
                    );
                    // Increase generic count
                    let curr_gargc = gargc;
                    println!("Generic typedef is now supported!");
                    gargc += 1;
                    Type::GenericArg(curr_gargc)
                } else {
                    Type::generic_from_ty(generic_ty, ctx)
                };
                let name = escape_field_name(&field.name.to_string());
                fields.push((name, ty));
            }
            inner_types.push(Self {
                access: AccessModifer::Public,
                name: variant_name.into(),
                inner_types: vec![],
                fields,
                functions: vec![],
                gargc,
                extends: None,
                explicit_offsets: None,
            });
        }
        res.push(Self {
            access,
            name,
            inner_types,
            fields,
            functions: vec![],
            gargc,
            extends: None,
            explicit_offsets: Some(explicit_offsets),
        });
        res
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
    } else if !name.chars().next().unwrap().is_alphabetic() || name == "value" || name == "flags" {
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
