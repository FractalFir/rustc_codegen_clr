use crate::IString;
use rustc_middle::ty::{
    AdtDef, FloatTy, GenericArg, Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy,
};
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
    // Foregin type. Will never be interacted with directly
    Foreign,
    GenericArg(u32),
    DotnetChar,
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
            assembly: assembly.map(std::convert::Into::into),
            name_path: name_path.into(),
            generics: Vec::new(),
        }
    }
    pub fn append_path(&mut self, append: &str) {
        let mut name_path = self.name_path.to_string();
        name_path.push_str(append);
        self.name_path = name_path.into();
    }
    pub fn asm(&self) -> Option<&str> {
        self.assembly.as_ref().map(std::convert::AsRef::as_ref)
    }
    pub fn name_path(&self) -> &str {
        self.name_path.as_ref()
    }
    pub fn generics(&self) -> &[Type] {
        self.generics.as_ref()
    }
    pub fn set_generics(&mut self, generics: impl Into<Vec<Type>>) {
        self.generics = generics.into();
    }
    pub fn set_generics_identity(&mut self) {
        self.generics = crate::type_def::ident_gargs(self.generics.len()).into();
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
        let name = crate::utilis::adt_name(adt);
        Self {
            assembly: None,
            name_path: name,
            generics,
        }
    }
}
impl Type {
    pub fn as_dotnet(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            _ => None,
        }
    }
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
            TyKind::Ref(_region, inner, _mut) => match inner.kind() {
                TyKind::Str => {
                    let str_type = DotnetTypeRef {
                        assembly: None,
                        name_path: "RustStr".into(),
                        generics: vec![],
                    };
                    Self::DotnetType(Box::new(str_type))
                }
                _ => {
                    println!("Ref kind {:?}", inner.kind());
                    Self::Ptr(Box::new(Self::from_ty(*inner, tyctx)))
                }
            },
            TyKind::Tuple(types) => {
                if types.is_empty() {
                    Type::Void
                } else {
                    todo!("Tuples are not supported yet!")
                }
            }
            TyKind::Slice(inner) => {
                let slice_tpe = DotnetTypeRef {
                    assembly: None,
                    name_path: "RustSlice".into(),
                    generics: vec![Self::from_ty(*inner, tyctx)],
                };
                Self::DotnetType(Box::new(slice_tpe))
            }
            TyKind::Never => Self::Void, // TODO: ensure this is always OK
            TyKind::Adt(adt_def, subst) => {
                Self::DotnetType(Box::new(DotnetTypeRef::from_adt(adt_def, subst, tyctx)))
            }
            TyKind::Dynamic(_, _, _) => Type::Unresolved,
            TyKind::Str => Type::Unresolved,
            TyKind::Foreign(_) => Type::Foreign,
            TyKind::Bound(_, _inner) => Type::Foreign,
            TyKind::FnPtr(_) => Type::USize,
            TyKind::Param(param_ty) => Type::GenericArg(param_ty.index),
            TyKind::Alias(_, alias_ty) => Self::from_ty(alias_ty.self_ty(), tyctx),
            //TyKind::Closure(def_id,subst)=>Self::from_ty(Instance::resolve(tyctx,ParamEnv::empty(),*def_id,subst).unwrap().unwrap().ty(tyctx,ParamEnv::empty()),tyctx),
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
