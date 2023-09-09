use crate::IString;
use rustc_middle::ty::{
    AdtDef, AdtKind, Const, FloatTy, GenericArg, IntTy, Ty, TyCtxt, TyKind, UintTy,
};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub(crate) struct FieldType {
    pub(crate) name: IString,
    pub(crate) tpe: Type,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub(crate) struct EnumVariant {
    pub(crate) name: IString,
    pub(crate) index: u32,
    pub(crate) fields: Vec<FieldType>,
}
pub(crate) fn genric_name(src_name:&str,params:&[Option<Type>])->IString{
    let param_string:String = params.iter().map(|tpe|
        if let Some(tpe) = tpe{crate::assembly_exporter::ilasm_exporter::type_cil(tpe).unwrap()}else{"Unresolved".into()}
    ).collect();
    format!("{src_name}{param_string}").into()
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub(crate) enum Type {
    //Intieger types
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    USize,
    ISize,
    //Float types
    F32,
    F64,
    //Referneces and pointers
    Ref(Box<Self>),
    Ptr(Box<Self>),
    //Algebraic Data Types
    Struct {
        name: IString,
        fields: Box<[FieldType]>,
    },
    Enum {
        name: IString,
        variants: Box<[EnumVariant]>,
    },
    //Slice/Array types
    StrSlice,
    Slice(Box<Self>),
    Array {
        element: Box<Self>,
        length: u64,
    },
    //Genrics
    GenericParam {
        index: u32,
    },
    //Special types
    Bool,
    Tuple(Box<[Self]>),
    Void,
    ExternType {
        asm: IString,
        name: IString,
    },
}
pub(crate) trait ToCLRType {
    fn clr_tpe() -> Type;
}
macro_rules! clr_tpe_map {
    ($tpe:ty,$clr_type:expr) => {
        impl ToCLRType for $tpe {
            fn clr_tpe() -> Type {
                $clr_type
            }
        }
    };
}
//Ints
clr_tpe_map! {i8,Type::I8}
clr_tpe_map! {u8,Type::U8}
clr_tpe_map! {i16,Type::I16}
clr_tpe_map! {u16,Type::U16}
clr_tpe_map! {i32,Type::I32}
clr_tpe_map! {u32,Type::U32}
clr_tpe_map! {i64,Type::I64}
clr_tpe_map! {u64,Type::U64}
clr_tpe_map! {i128,Type::I128}
clr_tpe_map! {u128,Type::U128}
clr_tpe_map! {isize,Type::ISize}
clr_tpe_map! {usize,Type::USize}
//Floats
clr_tpe_map! {f32,Type::F32}
clr_tpe_map! {f64,Type::F64}
//Pointers and referneces
impl<T: ToCLRType> ToCLRType for *const T {
    fn clr_tpe() -> Type {
        Type::Ptr(Box::new(T::clr_tpe()))
    }
}
impl<T: ToCLRType> ToCLRType for *mut T {
    fn clr_tpe() -> Type {
        Type::Ptr(Box::new(T::clr_tpe()))
    }
}
impl<T: ToCLRType> ToCLRType for &T {
    fn clr_tpe() -> Type {
        Type::Ref(Box::new(T::clr_tpe()))
    }
}
impl<T: ToCLRType> ToCLRType for &mut T {
    fn clr_tpe() -> Type {
        Type::Ref(Box::new(T::clr_tpe()))
    }
}
//Specials
clr_tpe_map! {(),Type::Void}
fn get_array_length(arr_len: Const) -> u64 {
    let scalar = arr_len
        .try_to_scalar()
        .expect("Non-scalar array lengths are not supported yet!");
    let value = scalar.to_u64().expect("Could not convert scalar to u64!");
    value
}
// Cpnversions from the Rust Type system
impl Type {
    /// Returns the type a pointer or a refernece points to.
    pub(crate) fn pointed_type(&self) -> Option<&Self> {
        match self {
            Self::Ptr(inner) => Some(inner),
            Self::Ref(inner) => Some(inner),
            _ => None,
        }
    }
    // Returns the type the element of the array or slice.
    pub(crate) fn element_type(&self) -> Option<&Self> {
        match self {
            Self::Array { element, .. } => Some(element.as_ref()),
            Self::Slice(element) => Some(element.as_ref()),
            _ => None,
        }
    }
    /// Returns true if type is [`Type::Void`]
    pub(crate) fn is_void(&self) -> bool {
        matches!(self, Self::Void)
    }
    pub(crate) fn box_from_ty<'ctx>(rust_tpe: &Ty<'ctx>, tyctx: &TyCtxt<'ctx>) -> Box<Self> {
        Box::new(Self::from_ty(rust_tpe, tyctx))
    }
    pub(crate) fn from_ty_non_cyclic<'ctx>(rust_tpe: &Ty<'ctx>, tyctx: &TyCtxt<'ctx>) -> Self {
        Self::from_ty(rust_tpe, tyctx)
    }
    pub(crate) fn from_ty<'ctx>(rust_tpe: &Ty<'ctx>, tyctx: &TyCtxt<'ctx>) -> Self {
        match rust_tpe.kind() {
            //Basic types
            TyKind::Bool => Self::Bool,
            TyKind::Int(int) => int.into(),
            TyKind::Uint(uint) => uint.into(),
            TyKind::Char => Self::U64,
            TyKind::Float(float) => float.into(),
            //Special
            TyKind::Never => Self::Void,
            TyKind::Str => Self::StrSlice,
            TyKind::Foreign(_) => Self::Void,
            TyKind::Array(element_type, length_const) => Self::Array {
                element: Box::new(Self::from_ty(element_type, tyctx)),
                length: get_array_length(*length_const),
            },
            TyKind::RawPtr(ptr_info) => Self::Ptr(Self::box_from_ty(&ptr_info.ty, tyctx)),
            TyKind::Ref(_, referenced_type, _) => {
                Self::Ref(Self::box_from_ty(referenced_type, tyctx))
            }
            TyKind::FnDef(_, _) => Type::ISize, //todo!("Function types are not supported yet."),
            TyKind::FnPtr(_) => Type::ISize, //todo!("Function pointer types are not supported yet. FnPtr:{rust_tpe:?}"),
            TyKind::Closure(_, _) => todo!("Closure types are not supported yet."),
            TyKind::Slice(element_type) => Self::Slice(Self::box_from_ty(element_type, tyctx)),
            TyKind::Dynamic(_, _, _) => Self::Void, //Dynamics are needed for `no_std` to work.
            TyKind::Generator(_, _, _) => todo!("Gernerators are not supported."),
            TyKind::GeneratorWitness(_) => todo!("Gernerators are not supported."),
            TyKind::GeneratorWitnessMIR { .. } => todo!("Gernerators are not supported."),
            TyKind::Tuple(elements) => {
                if elements.len() == 0 {
                    Self::Void
                } else {
                    Self::Tuple(elements.iter().map(|e| Self::from_ty(&e, tyctx)).collect())
                }
            }
            TyKind::Param(param) => Self::GenericParam { index: param.index },
            TyKind::Alias(_, aty) => Self::from_ty(&aty.to_ty(*tyctx), tyctx),
            TyKind::Infer(_) => {
                panic!("`rustc_codgen_clr` was passed an invalid type of kind `Infer`")
            }
            TyKind::Error(_) => {
                panic!("`rustc_codgen_clr` was passed an invalid type of kind `Error`")
            }
            TyKind::Placeholder(_) => {
                panic!("`rustc_codgen_clr` was passed an invalid type of kind `Placeholder`")
            }
            TyKind::Bound(_, _) => panic!("`rustc_codgen_clr` was passed a bound type."),
            TyKind::Adt(adt, subst) => Self::from_adt(adt, subst, tyctx),
            //_ => todo!("type:{rust_tpe}"),
        }
    }
    fn from_adt<'ctx>(
        adt: &AdtDef<'ctx>,
        subst: &[GenericArg<'ctx>],
        tyctx: &TyCtxt<'ctx>,
    ) -> Self {
        let subst: Box<[Option<_>]> = subst
            .iter()
            .map(|arg| Some(Self::from_ty(&arg.as_type()?, tyctx)))
            .collect();
        match adt.adt_kind() {
            AdtKind::Struct => {
                if adt.is_box() {
                    assert_eq!(
                        adt.all_fields().count(),
                        2,
                        "Box must have exactly two fields!"
                    );
                    let t_field = adt.all_fields().nth(0).unwrap();
                    let t_type = tyctx.type_of(t_field.did).skip_binder();
                    let mut res = Self::Ptr(Self::box_from_ty(&t_type, tyctx));
                    res.resolve(&subst);
                    return res;
                }
                let fields: Vec<FieldType> = adt
                    .all_fields()
                    .map(|field| FieldType {
                        name: field.name.to_string().into(),
                        tpe: Self::from_ty_non_cyclic(
                            &tyctx.type_of(field.did).skip_binder(),
                            tyctx,
                        ),
                    })
                    .collect();
                let name = adt_name(adt);
                let mut res = Self::Struct {
                    name,
                    fields: fields.into(),
                };
                res.resolve(&subst);
                res
            }
            AdtKind::Union => todo!("Can't yet handle unions"),
            AdtKind::Enum => {
                let name = adt_name(adt);
                let variants = adt
                    .variants()
                    .into_iter()
                    .enumerate()
                    .map(|(vidx, def)| {
                        let name = def.name.as_str().into();
                        let fields = def
                            .fields
                            .iter()
                            .map(|field| FieldType {
                                name: field.name.to_string().into(),
                                tpe: Self::from_ty_non_cyclic(
                                    &tyctx.type_of(field.did).skip_binder(),
                                    tyctx,
                                ),
                            })
                            .collect();
                        EnumVariant {
                            name,
                            index: vidx as u32,
                            fields,
                        }
                    })
                    .collect();
                Self::Enum {
                    name: name,
                    variants: variants,
                }
            }
        }
    }
    fn resolve(&mut self, params: &[Option<Type>]) {
        match self {
            Self::GenericParam { index } => match params[*index as usize].as_ref() {
                Some(tpe) => *self = tpe.clone(),
                None => (),
            },
            Type::U8
            | Type::I8
            | Type::U16
            | Type::I16
            | Type::U32
            | Type::I32
            | Type::F32
            | Type::U64
            | Type::I64
            | Type::F64
            | Type::I128
            | Type::U128
            | Type::ISize
            | Type::USize
            | Type::StrSlice
            | Type::Bool
            | Type::Void
            | Type::ExternType { .. } => (),
            Self::Tuple(inner) => inner.iter_mut().for_each(|p| p.resolve(params)),
            Self::Ptr(inner) => inner.resolve(params),
            Self::Ref(inner) => inner.resolve(params),
            Self::Slice(inner) => inner.resolve(params),
            Self::Struct { fields, name } =>{
                fields
                .iter_mut()
                .for_each(|field| field.tpe.resolve(params));
                *name = genric_name(name,params);
            },
            Self::Array { element, .. } => element.resolve(params),
            Self::Enum {  variants, name } => {
                *name = genric_name(name,params);
                variants.iter_mut().for_each(|variant| {
                variant
                    .fields
                    .iter_mut()
                    .for_each(|field| field.tpe.resolve(params))
            })},
        }
    }
    pub(crate) fn field(&self, variant: u32, field_index: u32) -> &FieldType {
        match self {
            Self::Struct {  fields,.. } => {
                assert_eq!(
                    variant, 0,
                    "Struct have only one variant, but variant is {variant}"
                );
                &fields[field_index as usize]
            }
            _ => todo!("type {self:?} is not type!"),
        }
    }
}
fn adt_name(adt: &AdtDef) -> IString {
    //TODO: find a better way to get adt name!
    format!("{adt:?}").into()
}
impl From<&IntTy> for Type {
    fn from(int_tpe: &IntTy) -> Self {
        match int_tpe {
            IntTy::I8 => Self::I8,
            IntTy::I16 => Self::I16,
            IntTy::I32 => Self::I32,
            IntTy::I64 => Self::I64,
            IntTy::I128 => Self::I128,
            IntTy::Isize => Self::ISize,
        }
    }
}
impl From<&UintTy> for Type {
    fn from(uint_tpe: &UintTy) -> Self {
        match uint_tpe {
            UintTy::U8 => Self::U8,
            UintTy::U16 => Self::U16,
            UintTy::U32 => Self::U32,
            UintTy::U64 => Self::U64,
            UintTy::U128 => Self::U128,
            UintTy::Usize => Self::USize,
        }
    }
}
impl From<&FloatTy> for Type {
    fn from(float: &FloatTy) -> Self {
        match float {
            FloatTy::F32 => Self::F32,
            FloatTy::F64 => Self::F64,
        }
    }
}
impl Eq for Type {}
