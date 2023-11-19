use super::{DotnetTypeRef, Type, TypeDef};
use crate::{
    access_modifier::AccessModifer, r#type::escape_field_name, utilis::enum_tag_size, IString,
};
use either::Either;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, AdtKind, Ty, TyCtxt, TyKind};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
enum GenericResolvePath {
    Subst { nth: u32 },
    Tuple { elements: Box<[GenericResolvePath]> },
    Concreate(Type),
    Ptr(Box<Self>),
    Ref(Box<Self>),
    DotnetRef(Box<(DotnetTypeRef, Box<[Option<Type>]>)>),
    Alias{projection_trait:DefId},
}
impl GenericResolvePath {
    fn tuple(types: &[Either<Type, GenericResolvePath>]) -> Either<Type, Self> {
        // If any type is not fully resolved, create a generic tuple type.
        if types.iter().any(|tpe| tpe.is_right()) {
            Either::Right(Self::Tuple {
                elements: types
                    .iter()
                    .map(|tpe| (*tpe).clone().into())
                    .collect::<Box<[GenericResolvePath]>>(),
            })
        } else {
            Either::Left(
                super::tuple_type(
                    &types
                        .iter()
                        .map(|tpe| tpe.clone().unwrap_left())
                        .collect::<Vec<_>>(),
                )
                .into(),
            )
        }
    }
    fn generalize(&self, generics: &mut Vec<GenericResolvePath>) -> Type {
        match self {
            Self::Ptr(inner) => Type::Ptr(inner.generalize(generics).into()),
            Self::Concreate(tpe) => tpe.clone(),
            Self::Subst { .. } | Self::Alias { .. }=> Type::GenericArg(generic_idx(generics, self)),
            _ => todo!("Can't generalize grp {self:?}"),
        }
    }
}
impl From<Either<Type, Self>> for GenericResolvePath {
    fn from(value: Either<Type, Self>) -> Self {
        match value {
            Either::Left(tpe) => GenericResolvePath::Concreate(tpe),
            Either::Right(right) => right,
        }
    }
}
impl GenericResolvePath {
    fn monomorphize_from(&self, subst: &[Option<Type>],cache:&TyCache) -> Type {
        match self {
            Self::Concreate(tpe) => tpe.clone(),
            Self::Subst { nth } => (&subst[*nth as usize])
                .clone()
                .expect("ERROR: non type subst where type expected!"),
            Self::Ptr(inner) => Type::Ptr(inner.monomorphize_from(subst,cache).into()),
            Self::Alias{projection_trait}=>cache.resolve_alias(*projection_trait,subst.into()),
            _ => todo!("Subst type {self:?} not supported yet!"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TypeDefAndGenericInfo {
    pub type_def: TypeDef,
    generic_info: Vec<GenericResolvePath>,
}
impl TypeDefAndGenericInfo {
    fn morphic_ref(&self, subst: &[Option<Type>],cache:&TyCache) -> DotnetTypeRef {
        let name = self.type_def.name();
        let mut dref = DotnetTypeRef::new(None, name);
        let generics: Vec<_> = self
            .generic_info
            .iter()
            .map(|gi| gi.monomorphize_from(subst,cache))
            .collect();
        if name.contains("Vec") {
            println!(
                "Vec generics:{generics:?} {generic_info:?}",
                generic_info = self.generic_info
            );
        }
        dref.set_generics(generics);
        dref
    }
}
fn generic_idx(generics: &mut Vec<GenericResolvePath>, generic: &GenericResolvePath) -> u32 {
    for (idx, generic_at) in generics.iter().enumerate() {
        if generic_at == generic {
            return idx as u32;
        }
    }
    let idx = generics.len();
    generics.push(generic.clone());
    idx as u32
}
fn generalize_type(
    generics: &mut Vec<GenericResolvePath>,
    tpe: Either<Type, GenericResolvePath>,
) -> Type {
    match tpe {
        Either::Left(tpe) => tpe,
        Either::Right(generic) => generic.generalize(generics),
    }
}
pub struct TyCache {
    type_def_cache: HashMap<DefId, TypeDefAndGenericInfo>,
    //TODO: Alias cache should take into accunt not only type generic, but also const generics and such. 
    alias_cache:HashMap<(Vec<Option<Type>>,DefId),Type>,
}
impl TyCache {
    pub fn empty() -> Self {
        Self {
            type_def_cache: HashMap::new(),
            alias_cache: HashMap::new(),
        }
    }
    fn resolve_alias(&self,def_id:DefId,subst:Vec<Option<Type>>)->Type{
        self.alias_cache.get(&(subst,def_id)).expect("ERROR: unresolved type alias! TyCache could not resolve a type alias, because it did not have it in its database.").clone()
    }
    fn struct_<'tyctx>(
        &mut self,
        adt: AdtDef<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDefAndGenericInfo {
        let name = crate::utilis::adt_name(&adt);
        let mut fields = Vec::new();
        for field in adt.all_fields() {
            let name = escape_field_name(&field.name.to_string());
            let generic_ty = tyctx.type_of(field.did).instantiate_identity();
            let generic_ty = self.generic_type_from_cache(generic_ty, tyctx);
            fields.push((name, generic_ty));
        }
        let mut generic_info = Vec::new();
        let fields: Vec<_> = fields
            .into_iter()
            .map(|(name, tpe)| (name, generalize_type(&mut generic_info, tpe)))
            .collect();

        let access = AccessModifer::Public;
        let type_def = TypeDef::new(
            access,
            name,
            vec![],
            fields,
            vec![],
            None,
            generic_info.len() as u32,
            None,
        );
        //todo!("Can't yet create typedefs for struct {adt:?}\n fields:{fields:?}");
        TypeDefAndGenericInfo {
            type_def,
            generic_info,
        }
    }
    fn enum_<'tyctx>(
        &mut self,
        adt: AdtDef<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDefAndGenericInfo {
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
                let generic_ty = tyctx.type_of(field.did).instantiate_identity();
                let name = escape_field_name(&field.name.to_string());

                let generic_ty = self.generic_type_from_cache(generic_ty, tyctx);
                variant_fields.push((name, generic_ty));
            }
            variants.push((variant_name, variant_fields));
        }
        let mut inner_types = vec![];
        let mut generic_info = Vec::new();
        let variants: Vec<_> = variants
            .into_iter()
            .map(|(variant_name, fields)| {
                let fields: Vec<_> = fields
                    .into_iter()
                    .map(|(name, tpe)| (name, generalize_type(&mut generic_info, tpe)))
                    .collect();
                (variant_name, fields)
            })
            .collect();

        let enum_name = crate::utilis::adt_name(&adt);
        for (variant_name, field_list) in variants {
            let inner = TypeDef::new(
                access,
                variant_name.clone(),
                vec![],
                field_list,
                vec![],
                None,
                generic_info.len() as u32,
                None,
            );
            let mut dref = DotnetTypeRef::new(None, &format!("{enum_name}/{variant_name}"));
            let generics: Vec<_> = generic_info
                .iter()
                .enumerate()
                .map(|(idx, _)| Type::GenericArg(idx as u32))
                .collect();
            dref.set_generics(generics);
            let variant_name: IString = format!("v_{variant_name}").into();
            fields.push((variant_name, dref.into()));
            inner_types.push(inner);
        }
        let type_def = TypeDef::new(
            access,
            enum_name,
            inner_types,
            fields,
            vec![],
            Some(explicit_offsets),
            generic_info.len() as u32,
            None,
        );
        TypeDefAndGenericInfo {
            type_def,
            generic_info,
        }
    }
    pub fn defs<'a>(&'a self) -> impl Iterator<Item = &TypeDef> + 'a {
        self.type_def_cache.values().map(|tdagi| &tdagi.type_def)
    }
    fn union_<'tyctx>(
        &mut self,
        adt: AdtDef<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDefAndGenericInfo {
        let mut fields = Vec::new();
        for field in adt.all_fields() {
            let name = escape_field_name(&field.name.to_string());
            let generic_ty = tyctx.type_of(field.did).instantiate_identity();
            let generic_ty = self.generic_type_from_cache(generic_ty, tyctx);
            fields.push((name, generic_ty));
        }
        let mut generic_info = Vec::new();
        let fields: Vec<_> = fields
            .into_iter()
            .map(|(name, tpe)| (name, generalize_type(&mut generic_info, tpe)))
            .collect();
        let name = crate::utilis::adt_name(&adt);
        let access = AccessModifer::Public;
        let explicit_offsets = (&fields).iter().map(|_| 0).collect();
        let type_def = TypeDef::new(
            access,
            name,
            vec![],
            fields,
            vec![],
            Some(explicit_offsets),
            generic_info.len() as u32,
            None,
        );
        //todo!("Can't yet create typedefs for struct {adt:?}\n fields:{fields:?}");
        TypeDefAndGenericInfo {
            type_def,
            generic_info,
        }
    }
    fn adt_<'tyctx>(
        &mut self,
        adt: AdtDef<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDefAndGenericInfo {
        match adt.adt_kind() {
            AdtKind::Struct => self.struct_(adt, tyctx),
            AdtKind::Enum => self.enum_(adt, tyctx),
            AdtKind::Union => self.union_(adt, tyctx),
        }
    }
    fn adt_from_cache<'tyctx>(
        &mut self,
        adt: AdtDef<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> TypeDefAndGenericInfo {
        let did = adt.did();
        let tdag = self.type_def_cache.get(&did);
        if let Some(tdag) = tdag {
            return tdag.clone();
        }
        let def = self.adt_(adt, tyctx);
        self.type_def_cache.insert(did, def);
        self.type_def_cache
            .get(&did)
            .expect("ERROR: Get directly after insert failed")
            .clone()
    }
    pub fn type_def_from_cache<'tyctx>(
        &mut self,
        ty: Ty<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> Option<TypeDefAndGenericInfo> {
        match ty.kind() {
            TyKind::Adt(adt, _) => Some(self.adt_from_cache(*adt, tyctx)),
            _ => todo!("Can't retrive typedef for type {ty:?} from cache yet!"),
        }
    }
    fn generic_type_from_cache<'tyctx>(
        &mut self,
        ty: Ty<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> Either<Type, GenericResolvePath> {
        match ty.kind() {
            TyKind::Bool => Either::Left(Type::Bool),
            TyKind::Int(int) => Either::Left(int.into()),
            TyKind::Uint(uint) => Either::Left(uint.into()),
            TyKind::Char => Either::Left(Type::U64),
            TyKind::Float(float) => Either::Left(float.into()),
            TyKind::Tuple(types) => {
                let types: Vec<_> = types
                    .iter()
                    .map(|ty| self.generic_type_from_cache(ty, tyctx))
                    .collect();
                if types.is_empty() {
                    Either::Left(Type::Void)
                } else {
                    GenericResolvePath::tuple(&types)
                }
            }
            TyKind::Never => Either::Left(Type::Void), // TODO: ensure this is always OK
            TyKind::RawPtr(type_and_mut) => {
                let ptr = self.generic_type_from_cache(type_and_mut.ty, tyctx);
                match ptr {
                    Either::Left(tpe) => Either::Left(Type::Ptr(tpe.into())),
                    Either::Right(generic) => {
                        Either::Right(GenericResolvePath::Ptr(generic.into()))
                    }
                }
            }
            TyKind::Adt(def, subst) => {
                let name = crate::utilis::adt_name(&def);
                if super::is_name_magic(name.as_ref()) {
                    return Either::Left(super::magic_type(name.as_ref(), def, subst, tyctx));
                }
                let subst: Vec<_> = subst
                    .iter()
                    .map(|garg| {
                        let ty = garg.as_type();
                        match ty {
                            Some(ty) => Some(self.generic_type_from_cache(ty, tyctx)),
                            None => None,
                        }
                    })
                    .collect();
                let def_and_generic = self.adt_from_cache(*def, tyctx);
                if subst.iter().any(|subst|subst.as_ref().is_some_and(|subst|subst.is_right())){
                    todo!("Handle generic ADT subst!")
                }
                else{
                    let subst:Vec<_> = subst.into_iter().map(|option|match option{
                        Some(inner)=>Some(inner.unwrap_left()),
                        None=>None,
                    }).collect();
                    Either::Left(Type::DotnetType(def_and_generic.morphic_ref(&subst,self).into()))
                }
            }
            TyKind::Dynamic(trait_, _, dyn_kind) => {
                println!("trait:{trait_:?} dyn_kind:{dyn_kind:?}");
                Either::Left(Type::Unresolved)
            }
            TyKind::Param(param_ty) => Either::Right(GenericResolvePath::Subst {
                nth: param_ty.index,
            }),
            TyKind::Ref(_region, inner, _mut) => match inner.kind() {
                TyKind::Slice(inner) => match self.generic_type_from_cache(*inner, tyctx) {
                    Either::Left(concreate) => {
                        let mut dotnet =
                            DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents");
                        dotnet.set_generics(vec![concreate.clone(), Type::USize]);
                        Either::Left(dotnet.into())
                    }
                    Either::Right(_generic) => todo!("Can't handle generic slice!"),
                },
                TyKind::Str => {
                    let mut dotnet = DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents");
                    dotnet.set_generics(vec![Type::U8, Type::USize]);
                    Either::Left(dotnet.into())
                }
                _ => match self.generic_type_from_cache(*inner, tyctx) {
                    Either::Left(tpe) => Either::Left(Type::Ptr(tpe.into())),
                    Either::Right(tpe) => Either::Right(GenericResolvePath::Ptr(tpe.into())),
                },
            },
            TyKind::Foreign(foregin) => {
                println!("foregin:{foregin:?}");
                Either::Left(Type::Foreign)
            }
            TyKind::Bound(_, _inner) => Either::Left(Type::Foreign),
            TyKind::FnPtr(_) => Either::Left(Type::USize),
            TyKind::Slice(inner) => match self.generic_type_from_cache(*inner, tyctx) {
                Either::Left(inner) => {
                    let mut slice_tpe = DotnetTypeRef::new(None, "RustSlice".into());
                    slice_tpe.set_generics(vec![inner]);
                    Either::Left(Type::DotnetType(Box::new(slice_tpe)))
                }
                Either::Right(generic) => todo!("Generic slice"),
            },
            TyKind::Array(element, length) => {
                //let length = crate::utilis::monomorphize(method, *length, tyctx);
                let length = crate::utilis::try_resolve_const_size(&length).unwrap();
                match self.generic_type_from_cache(*element, tyctx) {
                    Either::Left(element) => {
                        Either::Left(DotnetTypeRef::array(element, length).into())
                    }
                    Either::Right(tpe) => todo!("Generic array!"), //Either::Right(GenericResolvePath::Ptr(tpe.into())),
                }
            }
            TyKind::Alias(kind,ty)=>{
                //TODO: handle kind!
                let def = ty.def_id;
                /* 
                let generics:Vec<_> = ty.args.iter().map(|g|match g.as_type(){
                    Some(generic_arg)=>Some(self.generic_type_from_cache(generic_arg, tyctx)),
                    None=>None,
                }).collect();*/
                Either::Right(GenericResolvePath::Alias { projection_trait: def })
            }
            _ => todo!("Can't yet get type {ty:?} from type cache."),
        }
    }
    pub fn type_from_cache<'tyctx>(&mut self, ty: Ty<'tyctx>, tyctx: TyCtxt<'tyctx>) -> Type {
        if let Either::Left(tpe) = self.generic_type_from_cache(ty, tyctx) {
            tpe
        } else {
            panic!("Tried to get a morphic type from cache, but passed a generic {ty:?}!");
        }
    }
}
