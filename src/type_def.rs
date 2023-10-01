use crate::{
    access_modifier::AccessModifer,
    r#type::{DotnetTypeRef, Type},
    utilis::{enum_tag_size, tag_from_enum_variants},
    IString,
};
use rustc_middle::ty::{AdtDef, AdtKind, GenericArg, List, Ty, TyCtxt, TyKind};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TypeDef {
    access: AccessModifer,
    name: IString,
    inner_types: Vec<Self>,
    fields: Vec<(IString, Type)>,
    explicit_offsets: Option<Vec<u32>>,
    gargc: u32,
    extends: Option<DotnetTypeRef>,
}
impl TypeDef {
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
    pub fn inner_types(&self) -> &[Self] {
        &self.inner_types
    }
    pub fn explicit_offsets(&self) -> Option<&Vec<u32>> {
        self.explicit_offsets.as_ref()
    }
    pub fn nameonly(name: &str) -> Self {
        Self {
            access: AccessModifer::Public,
            name: name.into(),
            inner_types: vec![],
            fields: vec![],
            gargc: 0,
            extends: None,
            explicit_offsets: None,
        }
    }
    pub fn from_ty<'tycxt>(ty: Ty<'tycxt>, ctx: TyCtxt<'tycxt>) -> Vec<Self> {
        match ty.kind() {
            TyKind::Adt(adt_def, subst) => {
                let name = crate::utilis::adt_name(adt_def);
                let gargc = subst.len() as u32;
                let access = AccessModifer::Public;
                match adt_def.adt_kind() {
                    AdtKind::Struct => Self::struct_from_adt(ty, adt_def, subst, ctx),
                    AdtKind::Enum => Self::enum_from_adt(ty, adt_def, subst, ctx),
                    AdtKind::Union => Self::union_from_adt(ty, adt_def, subst, ctx),
                }
            }
            TyKind::Ref(_region, inner, _mut) => Self::from_ty(*inner, ctx),
            TyKind::RawPtr(inner_and_mut) => Self::from_ty(inner_and_mut.ty, ctx),
            TyKind::Slice(inner) => Self::from_ty(*inner, ctx),
            TyKind::Array(element, size) => {
                let length = crate::utilis::try_resolve_const_size(size)
                    .expect("Could not resolve array size!");
                let name = format!("Arr{length}");
                let mut fields = Vec::with_capacity(length);
                for field in 0..length {
                    fields.push((format!("f_{field}").into(), Type::GenericArg(0)))
                }
                let def = TypeDef {
                    access: AccessModifer::Public,
                    name: name.into(),
                    inner_types: vec![],
                    fields,
                    explicit_offsets: None,
                    gargc: 1,
                    extends: None,
                };
                let mut types = Self::from_ty(*element, ctx);
                types.push(def);
                types
            }
            TyKind::Alias(_, alias_ty) => {
                let alias_ty = ctx.type_of(alias_ty.def_id).instantiate_identity();
                Self::from_ty(alias_ty, ctx)
            }
            _ => vec![],
        }
    }
    fn struct_from_adt<'tcx>(
        original: Ty<'tcx>,
        adt_def: &AdtDef<'tcx>,
        subst: &'tcx List<GenericArg<'tcx>>,
        ctx: TyCtxt<'tcx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx));
            }
        });
        for field in adt_def.all_fields() {
            //rustc_middle::ty::List::empty()
            let ty = ctx.type_of(field.did).instantiate_identity();
            let ty = Type::from_ty(ty, ctx);
            let name = escape_field_name(&field.name.to_string());
            fields.push((name, ty));
        }
        res.push(Self {
            access,
            name,
            inner_types: vec![],
            fields,
            gargc,
            extends: None,
            explicit_offsets: None,
        });
        res
    }
    fn union_from_adt<'tcx>(
        original: Ty<'tcx>,
        adt_def: &AdtDef<'tcx>,
        subst: &'tcx List<GenericArg<'tcx>>,
        ctx: TyCtxt<'tcx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx));
            }
        });
        for field in adt_def.all_fields() {
            //rustc_middle::ty::List::empty()
            let ty = ctx.type_of(field.did).instantiate_identity();
            let ty = Type::from_ty(ty, ctx);
            let name = escape_field_name(&field.name.to_string());
            fields.push((name, ty));
        }
        let explicit_offsets = Some(fields.iter().map(|_| 0).collect());
        res.push(Self {
            access,
            name,
            inner_types: vec![],
            fields,
            gargc,
            extends: None,
            explicit_offsets,
        });
        res
    }
    fn enum_from_adt<'tcx>(
        original: Ty<'tcx>,
        adt_def: &AdtDef<'tcx>,
        subst: &'tcx List<GenericArg<'tcx>>,
        ctx: TyCtxt<'tcx>,
    ) -> Vec<Self> {
        let name = crate::utilis::adt_name(adt_def);
        // Handle  `Never` type alias
        if name.to_string() == "std.convert.Infallible" {
            return vec![];
        }
        let gargc = subst.len() as u32;
        let access = AccessModifer::Public;
        //let mut fields = Vec::with_capacity(adt_def.all_fields().count());
        let mut res = Vec::new();
        adt_def.all_fields().for_each(|field| {
            let resolved_field_ty = field.ty(ctx, subst);
            //This is a simple loop prevention. More complex types may still lead to cycles. TODO: deal with cycles.
            if resolved_field_ty != original {
                res.extend(Self::from_ty(resolved_field_ty, ctx));
            }
        });
        let mut fields = vec![(
            "_tag".into(),
            tag_from_enum_variants(adt_def.variants().len() as u64),
        )];
        let mut explicit_offsets = vec![0];
        let tag_size = enum_tag_size(adt_def.variants().len() as u64);
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
                let ty = ctx.type_of(field.did).instantiate_identity();
                let ty = Type::from_ty(ty, ctx);
                let name = escape_field_name(&field.name.to_string());
                fields.push((name, ty));
            }
            inner_types.push(Self {
                access: AccessModifer::Public,
                name: variant_name.into(),
                inner_types: vec![],
                fields,
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
            gargc,
            extends: None,
            explicit_offsets: Some(explicit_offsets),
        });
        res
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
