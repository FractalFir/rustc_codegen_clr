use crate::IString;
use rustc_middle::{
    mir::Mutability,
    ty::{FloatTy, IntTy, Ty, TyKind, UintTy},
};
use rustc_middle::ty::AdtKind;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub(crate) enum VariableType {
    Void,
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    F32,
    F64,
    Bool,
    Ref(Box<Self>),
    RefMut(Box<Self>),
    Array{
       element:Box<Self>,
       length:usize,
    },
    Struct(IString)
}

impl VariableType {
    pub(crate) fn is_void(&self) -> bool {
        matches!(self, Self::Void)
    }
    pub(crate) fn get_pointed_type(&self) -> Option<Self> {
        match self {
            Self::Ref(inner) => Some((*inner.as_ref()).clone()),
            Self::RefMut(inner) => Some((*inner.as_ref()).clone()),
            _ => None,
        }
    }
    pub(crate) fn from_ty(ty: Ty) -> Self {
        match ty.kind() {
            TyKind::Int(IntTy::I8) => VariableType::I8,
            TyKind::Int(IntTy::I16) => VariableType::I16,
            TyKind::Int(IntTy::I32) => VariableType::I32,
            TyKind::Int(IntTy::I64) => VariableType::I64,
            TyKind::Int(IntTy::I128) => VariableType::I128,
            TyKind::Int(IntTy::Isize) => VariableType::ISize,
            TyKind::Uint(UintTy::U8) => VariableType::U8,
            TyKind::Uint(UintTy::U16) => VariableType::U16,
            TyKind::Uint(UintTy::U32) => VariableType::U32,
            TyKind::Uint(UintTy::U64) => VariableType::U64,
            TyKind::Uint(UintTy::U128) => VariableType::U128,
            TyKind::Uint(UintTy::Usize) => VariableType::USize,
            TyKind::Float(FloatTy::F32) => VariableType::F32,
            TyKind::Float(FloatTy::F64) => VariableType::F64,
            TyKind::Bool => VariableType::Bool,
            TyKind::Char => todo!("Can't handle chars yet!"),
            TyKind::Foreign(_ftype) => todo!("Can't handle foreign types yet!"),
            TyKind::Str => todo!("Can't handle string slices yet!"),
            TyKind::Array(element_type, length) => Self::Array{element:Box::new(Self::from_ty(*element_type)),length:{
                let scalar = length.try_to_scalar().expect("Could not convert the scalar");
                let value = scalar.to_u64().expect("Could not convert scalar to u64!");
                value as usize
            }},
            TyKind::Slice(_element_type) => todo!("Can't handle slices yet!"),
            TyKind::Adt(adt_def, _subst) =>{
                let adt = adt_def;
                //let tcxt:&_ = adt.0.0;
                //TODO: Figure out a better way to get this name!
                let name = format!("{adt:?}");//tcxt.get_diagnostic_name();
                match adt.adt_kind(){
                    AdtKind::Struct => VariableType::Struct(name.into()),
                    AdtKind::Union => todo!("Can't yet handle unions"),
                    AdtKind::Enum => todo!("Can't yet handle enum"),
                }   
            }
            TyKind::RawPtr(_target_type) => todo!("Can't handle pointers yet!"),
            TyKind::FnPtr(_sig) => todo!("Can't handle function pointers yet!"),
            TyKind::Ref(region, ref_type, mutability) => {
                // There is no such concept as lifetimes in CLR
                let _ = region;
                match mutability {
                    Mutability::Mut => Self::RefMut(Box::new(Self::from_ty(*ref_type))),
                    Mutability::Not => Self::Ref(Box::new(Self::from_ty(*ref_type))),
                }
            }
            TyKind::Bound(debrujin_index, bound_ty) => {
                todo!("Bound, debrujin_index:{debrujin_index:?}, bound_ty:{bound_ty:?}");
            }
            TyKind::Tuple(inner_types) => {
                if inner_types.len() == 0 {
                    return Self::Void;
                }
                todo!("Can't handle tuples yet!");
            }
            _ => todo!("Unhandled type kind {:?}", ty.kind()),
        }
    }
    pub(crate) fn il_name(&self) -> IString {
        match self {
            Self::Void => "void".into(),
            Self::I8 => "int8".into(),
            Self::I16 => "int16".into(),
            Self::I32 => "int32".into(),
            Self::I64 => "int64".into(),
            Self::I128 => "[System.Runtime]System.Int128".into(),
            Self::ISize => "native int".into(),
            Self::U8 => "uint8".into(),
            Self::U16 => "uint16".into(),
            Self::U32 => "uint32".into(),
            Self::U64 => "uint64".into(),
            Self::U128 => "[System.Runtime]System.UInt128".into(),
            Self::USize => "native uint".into(),
            Self::F32 => "float32".into(),
            Self::F64 => "float64".into(),
            Self::Bool => "bool".into(),
            Self::Ref(inner) => format!("{inner}*", inner = inner.il_name()),
            Self::RefMut(inner) => format!("{inner}*", inner = inner.il_name()),
            Self::Struct(name) => (*name).clone().into(),
            Self::Array{element,length} => format!(
                "'RArray_{element_il}_{length}'",
                element_il = element.il_name().replace('\'',"")
            ).into(),
        }
        .into()
    }
}
