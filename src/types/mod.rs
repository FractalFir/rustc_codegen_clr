use crate::IString;
use rustc_middle::ty::{Const, FloatTy, IntTy, Ty, TyKind, UintTy};
pub(crate) struct FieldType {
    name: IString,
    tpe: Type,
}
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
        fields: Vec<FieldType>,
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
    ResolvedGenric {
        inner: Box<Self>,
        params: Vec<Self>,
    },
    //Special types
    Bool,
    Void,
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
impl From<Ty<'_>> for Type {
    fn from(value: Ty<'_>) -> Self {
        (&value).into()
    }
}
impl From<&Ty<'_>> for Type {
    fn from(rust_tpe: &Ty) -> Self {
        match rust_tpe.kind() {
            TyKind::Bool => Self::Bool,
            TyKind::Int(int) => int.into(),
            TyKind::Uint(uint) => uint.into(),
            TyKind::Char => Self::U64,
            TyKind::Float(float) => float.into(),
            TyKind::Str => Self::StrSlice,
            TyKind::Foreign(_) => Self::Void,
            TyKind::Array(element_type, length_const) => Self::Array {
                element: Box::new(element_type.into()),
                length: get_array_length(*length_const),
            },
            TyKind::RawPtr(ptr_info) => Self::Ptr(Box::new(ptr_info.ty.into())),
            TyKind::Ref(_, referenced_type, _) => Self::Ref(Box::new(referenced_type.into())),
            TyKind::FnDef(_, _) => todo!("Function types are not supported yet."),
            TyKind::FnPtr(_) => todo!("Function pointer types are not supported yet."),
            TyKind::Closure(_, _) => todo!("Closure types are not supported yet."),
            TyKind::Slice(element_type) => Self::Slice(Box::new(element_type.into())),
            _ => todo!("type:{rust_tpe}"),
        }
    }
}
impl From<&IntTy> for Type {
    fn from(int_tpe: &IntTy) -> Self {
        match int_tpe {
            IntTy::I8 => Self::I8,
            IntTy::I16 => Self::I16,
            IntTy::I32 => Self::I32,
            IntTy::I64 => Self::I64,
            IntTy::I128 => Self::I64,
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
            UintTy::U128 => Self::U64,
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
