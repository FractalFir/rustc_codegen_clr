use super::{DotnetTypeRef, Type, TypeDef};
use crate::{
    access_modifier::AccessModifer, r#type::escape_field_name, utilis::enum_tag_size, IString,
};
use either::Either;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, AdtKind, GenericArg, List, Ty, TyCtxt, TyKind};
use std::collections::HashMap;
pub struct TyCache {
    type_def_cache: HashMap<IString, TypeDef>,
}
impl TyCache {
    pub fn empty() -> Self {
        Self {
            type_def_cache: HashMap::new(),
        }
    }
    pub fn defs(&self) -> impl Iterator<Item = &TypeDef> {
        self.type_def_cache.values()
    }
    pub fn type_def_from_cache<'tyctx>(
        &mut self,
        ty: Ty<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> &TypeDef {
        match ty.kind() {
            TyKind::Adt(adt, susbt) => {
                let name = crate::utilis::adt_name(&adt);
                if super::is_name_magic(name.as_ref()) {
                    todo!("Can't yet get fields of interop types!");
                }
                let name = super::mangle_susbt(&name, susbt);
                if self.type_def_cache.get(name.as_ref()).is_none() {
                    self.type_from_cache(ty, tyctx);
                }
                self.type_def_cache
                    .get(name.as_ref())
                    .expect("Added type, but it is missing??")
            }
            _ => todo!("Can't retrive typedef for type {ty:?} from cache yet!"),
        }
    }
    fn adt<'tyctx>(
        &mut self,
        name: &str,
        def: AdtDef<'tyctx>,
        subst: &'tyctx List<rustc_middle::ty::GenericArg<'tyctx>>,
        tyctx: TyCtxt<'tyctx>,
    ) -> DotnetTypeRef {
        if self.type_def_cache.get(name).is_some() {
            return DotnetTypeRef::new(None, name.into());
        }
        let def = match def.adt_kind() {
            AdtKind::Struct => self.struct_(name, def, subst, tyctx),
            AdtKind::Enum => self.enum_(name, def, subst, tyctx),
            _ => todo!("adt {def:?} not supported!"),
        };
        self.type_def_cache.insert(name.into(), def);
        DotnetTypeRef::new(None, name.into())
    }
    fn struct_<'tyctx>(
        &mut self,
        name: &str,
        adt: AdtDef<'tyctx>,
        subst: &'tyctx List<rustc_middle::ty::GenericArg<'tyctx>>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDef {
        let mut fields = Vec::new();
        for field in adt.all_fields() {
            let name = escape_field_name(&field.name.to_string());
            let field_ty = self.type_from_cache(field.ty(tyctx, subst), tyctx);
            fields.push((name, field_ty));
        }

        let access = AccessModifer::Public;

        let type_def = TypeDef::new(access, name.into(), vec![], fields, vec![], None, 0, None);
        type_def
    }
    fn enum_<'tyctx>(
        &mut self,
        enum_name: &str,
        adt: AdtDef<'tyctx>,
        subst: &'tyctx List<rustc_middle::ty::GenericArg<'tyctx>>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDef {
        let access = AccessModifer::Public;
        let mut explicit_offsets: Vec<u32> = vec![0];

        let tag_size = enum_tag_size(adt.variants().len() as u64);
        let mut fields = vec![(
            "_tag".into(),
            crate::utilis::tag_from_enum_variants(adt.variants().len() as u64),
        )];
        explicit_offsets.extend(adt.variants().iter().map(|_| tag_size));
        //let mut inner_types = vec![];
        let mut variants = vec![];
        for variant in adt.variants() {
            let variant_name: IString = variant.name.to_string().into();
            let mut variant_fields = vec![];
            for field in &variant.fields {
                let name = escape_field_name(&field.name.to_string());
                let field_ty = self.type_from_cache(field.ty(tyctx, subst), tyctx);
                variant_fields.push((name, field_ty));
            }
            variants.push((variant_name, variant_fields));
        }
        let mut inner_types = vec![];

        for (variant_name, field_list) in variants {
            let inner = TypeDef::new(
                access,
                variant_name.clone(),
                vec![],
                field_list,
                vec![],
                None,
                0,
                None,
            );
            let dref = DotnetTypeRef::new(None, &format!("{enum_name}/{variant_name}"));
            let variant_name: IString = format!("v_{variant_name}").into();
            fields.push((variant_name, dref.into()));
            inner_types.push(inner);
        }
        let type_def = TypeDef::new(
            access,
            enum_name.into(),
            inner_types,
            fields,
            vec![],
            Some(explicit_offsets),
            0,
            None,
        );
        type_def
    }
    pub fn type_from_cache<'tyctx>(&mut self, ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> Type {
        match ty.kind() {
            TyKind::Bool => Type::Bool,
            TyKind::Int(int) => int.into(),
            TyKind::Uint(uint) => uint.into(),
            TyKind::Char => Type::U64,
            TyKind::Float(float) => float.into(),
            TyKind::Tuple(types) => {
                let types: Vec<_> = types
                    .iter()
                    .map(|ty| self.type_from_cache(ty, tyctx))
                    .collect();
                if types.is_empty() {
                    Type::Void
                } else {
                    super::tuple_type(&types).into()
                }
            }
            TyKind::Never => Type::Void, // TODO: ensure this is always OK
            TyKind::RawPtr(type_and_mut) => match type_and_mut.ty.kind() {
                TyKind::Slice(inner) => {
                    todo!("Can't handle slices!")
                }
                TyKind::Str => {
                    let mut dotnet = DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents");
                    dotnet.set_generics(vec![Type::U8, Type::USize]);
                    dotnet.into()
                }
                _ => Type::Ptr(self.type_from_cache(type_and_mut.ty, tyctx).into()),
            },
            TyKind::Adt(def, subst) => {
                let name = crate::utilis::adt_name(&def);
                if super::is_name_magic(name.as_ref()) {
                    return super::magic_type(name.as_ref(), def, subst, tyctx);
                }
                let mangled = super::mangle_susbt(&name, subst);
                println!("mangled:{mangled:?}");
                self.adt(&mangled, *def, subst, tyctx).into()
            }
            TyKind::Dynamic(trait_, _, dyn_kind) => {
                println!("trait:{trait_:?} dyn_kind:{dyn_kind:?}");
                Type::Unresolved
            }
            TyKind::Ref(_region, inner, _mut) => match inner.kind() {
                TyKind::Slice(inner) => {
                    let name: IString = format!(
                        "core.ptr.metadata.PtrComponents{}",
                        super::mangle_ty(*inner)
                    )
                    .into();
                    //let inner = self.inne
                    let dotnet = DotnetTypeRef::new(None, &name);
                    if self.type_def_cache.get_key_value(&name).is_some() {
                        return dotnet.into();
                    }
                    self.type_def_cache
                        .insert(name.clone(), TypeDef::ptr_components(&name, Type::USize));
                    dotnet.into()
                }
                TyKind::Str => {
                    let name = "core.ptr.metadata.PtrComponentsStr";
                    let dotnet = DotnetTypeRef::new(None, &name);
                    if self.type_def_cache.get_key_value(name).is_some() {
                        return dotnet.into();
                    }
                    self.type_def_cache
                        .insert(name.into(), TypeDef::ptr_components(&name, Type::USize));
                    dotnet.into()
                }
                _ => Type::Ptr(self.type_from_cache(*inner, tyctx).into()),
            },
            TyKind::Foreign(foregin) => {
                println!("foregin:{foregin:?}");
                Type::Foreign
            }
            TyKind::Bound(_, _inner) => Type::Foreign,
            TyKind::FnPtr(_) => Type::USize,
            TyKind::Slice(inner) => {
                //let match self.type_from_cache(*inner, tyctx)
                //let mut slice_tpe = DotnetTypeRef::new(None, "RustSlice".into());
                //slice_tpe.set_generics(vec![inner]);
                //Type::DotnetType(Box::new(slice_tpe)))
                todo!("Slice!")
            }
            TyKind::Array(element, length) => {
                //let length = crate::utilis::monomorphize(method, *length, tyctx);
                let length = crate::utilis::try_resolve_const_size(&length).unwrap();
                // let element = self.
                //DotnetTypeRef::array(element, length).into()
                todo!("Array!")
            }
            _ => todo!("Can't yet get type {ty:?} from type cache."),
        }
    }
}
