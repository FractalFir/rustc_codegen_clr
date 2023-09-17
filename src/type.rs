use crate::IString;
use rustc_middle::ty::{AdtDef, FloatTy, GenericArg, IntTy, Ty, TyCtxt, TyKind, UintTy};
/// This struct represetnts either a primitive .NET type (F32,F64), or stores information on how to lookup a more complex type (struct,class,array)
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub enum Type {
    Void,
    // Bool
    Bool,
    // Floating-point types
    F32,
    F64,
    // Unsigned intiegers
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    // Signed intiegers
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    // A refernece to a .NET type
    DotnetType(Box<DotnetTypeRef>),
    // Pointer to a type
    Ptr(Box<Self>),
    // Speical type marking an unresoved type. This is a work around some issues with corelib types. Nothing can ever interact directly with this type.
    Unresolved,
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub struct DotnetTypeRef {
    assembly: Option<IString>,
    name_path: IString,
    generics: Vec<Type>,
}
impl DotnetTypeRef {
    pub fn new(assembly: Option<&str>, name_path: &str) -> Self {
        Self {
            assembly: assembly.map(|asm| asm.into()),
            name_path: name_path.into(),
            generics: Vec::new(),
        }
    }
    pub fn asm(&self) -> Option<&str> {
        self.assembly.as_ref().map(|asm| asm.as_ref())
    }
    pub fn name_path(&self) -> &str {
        self.name_path.as_ref()
    }
    pub fn generics(&self) -> &[Type] {
        self.generics.as_ref()
    }
    fn from_adt<'ctx>(adt: &AdtDef<'ctx>, subst: &[GenericArg<'ctx>], tyctx: TyCtxt<'ctx>) -> Self {
        let generics: Vec<Type> = subst
            .iter()
            .map(|arg| {
                if let Some(resolved) = arg.as_type() {
                    Type::from_ty(resolved, tyctx)
                } else {
                    Type::Unresolved
                }
            })
            .collect();
        let name = adt_name(adt);
        Self {
            assembly: None,
            name_path: name,
            generics,
        }
    }
}
impl Type {
    pub fn from_ty<'ctx>(rust_tpe: Ty<'ctx>, tyctx: TyCtxt<'ctx>) -> Self {
        Self::from_ty_kind(rust_tpe.kind(), tyctx)
    }
    pub fn from_ty_kind<'ctx>(rust_tpe: &TyKind<'ctx>, tyctx: TyCtxt<'ctx>) -> Self {
        match rust_tpe {
            TyKind::Bool => Self::Bool,
            TyKind::Int(int) => int.into(),
            TyKind::Uint(uint) => uint.into(),
            TyKind::Char => Self::U64,
            TyKind::Float(float) => float.into(),
            TyKind::RawPtr(type_and_mut) => {
                Self::Ptr(Box::new(Self::from_ty(type_and_mut.ty, tyctx)))
            }
            TyKind::Ref(_region, inner, _mut) => Self::Ptr(Box::new(Self::from_ty(*inner, tyctx))),
            TyKind::Tuple(types) => {
                if types.is_empty() {
                    Type::Void
                } else {
                    todo!("Tuples are not supported yet!")
                }
            }
            TyKind::Never => Self::Void, // TODO: ensure this is always OK
            TyKind::Adt(adt_def, subst) => {
                Self::DotnetType(Box::new(DotnetTypeRef::from_adt(adt_def, subst, tyctx)))
            }
            _ => todo!("Unsupported type{rust_tpe:?}!"),
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
fn adt_name(adt: &AdtDef) -> IString {
    //TODO: find a better way to get adt name!
    format!("{adt:?}").replace("::", ".").into()
}
