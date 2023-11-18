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
    //Ref(Box<Self>),
    DotnetRef(Box<(DotnetTypeRef, Box<[Option<Type>]>)>),
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
    fn monomorphize_from(&self, subst: &[Option<Type>]) -> Type {
        match self {
            Self::Concreate(tpe) => tpe.clone(),
            Self::Subst { nth } => (&subst[*nth as usize])
                .clone()
                .expect("ERROR: non type subst where type expected!"),
            Self::Ptr(inner)=>Type::Ptr(inner.monomorphize_from(subst).into()),
            _ => todo!("Subst type {self:?} not supported yet!"),
        }
    }
}
#[derive(Debug, Clone)]
struct TypeDefAndGenericInfo {
    type_def: TypeDef,
    generic_info: Vec<GenericResolvePath>,
}
impl TypeDefAndGenericInfo {
    fn morphic_ref(&self, subst: &[Option<Type>]) -> DotnetTypeRef {
        let name = self.type_def.name();
        let mut dref = DotnetTypeRef::new(None, name);
        let generics: Vec<_> = self
            .generic_info
            .iter()
            .map(|gi| gi.monomorphize_from(subst))
            .collect();
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
        Either::Right(generic) => Type::GenericArg(generic_idx(generics, &generic)),
    }
}
pub struct TyCache {
    type_def_cache: HashMap<DefId, TypeDefAndGenericInfo>,
}
impl TyCache {
    pub fn empty() -> Self {
        Self {
            type_def_cache: HashMap::new(),
        }
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
        let mut fields = Vec::new();
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
    pub fn defs<'a>(&'a self)->impl Iterator<Item = &TypeDef> + 'a{
        self.type_def_cache.values().map(|tdagi|&tdagi.type_def)
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
    fn type_def_from_cache<'tyctx>(
        &mut self,
        ty: Ty<'tyctx>,
        tyctx: TyCtxt<'tyctx>,
    ) -> Option<TypeDefAndGenericInfo> {
        match ty.kind() {
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
                            Some(ty) => Some(self.type_from_cache(ty, tyctx)),
                            None => None,
                        }
                    })
                    .collect();
                let def_and_generic = self.adt_from_cache(*def, tyctx);
                Either::Left(Type::DotnetType(def_and_generic.morphic_ref(&subst).into()))
            }
            TyKind::Param(param_ty) => Either::Right(GenericResolvePath::Subst {
                nth: param_ty.index,
            }),
            TyKind::Ref(_region, inner, _mut) => match inner.kind() {
                TyKind::Slice(inner) => todo!("slice"), //Self::slice_ref(Type::generic_from_ty(*inner, tyctx)),
                TyKind::Str => todo!("slice"),
                _ => match self.generic_type_from_cache(*inner, tyctx) {
                    Either::Left(tpe) => Either::Left(Type::Ptr(tpe.into())),
                    Either::Right(tpe) => Either::Right(GenericResolvePath::Ptr(tpe.into())),
                },
            },
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
