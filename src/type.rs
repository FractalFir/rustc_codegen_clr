use crate::IString;
use rustc_middle::ty::{
    AdtDef, ClosureKind, ConstKind, FloatTy, GenericArg, Instance, IntTy, ParamEnv, Ty, TyCtxt,
    TyKind, UintTy,
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
    DotnetArray(Box<DotnetArray>),
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
pub struct DotnetArray {
    pub element: Type,
    pub dimensions: u64,
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
    pub fn array(element: Type, length: usize) -> Self {
        let name = format!("Arr{length}");
        let mut array = DotnetTypeRef::new(None, &name);
        array.set_generics([element]);
        array
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
                let name = crate::utilis::adt_name(adt_def);
                if is_name_magic(name.as_ref()) {
                    magic_type(name.as_ref(), adt_def, subst, tyctx)
                } else {
                    Self::DotnetType(Box::new(DotnetTypeRef::from_adt(adt_def, subst, tyctx)))
                }
            }
            TyKind::Dynamic(_, _, _) => Type::Unresolved,
            TyKind::Str => Type::Unresolved,
            TyKind::Foreign(_) => Type::Foreign,
            TyKind::Bound(_, _inner) => Type::Foreign,
            TyKind::FnPtr(_) => Type::USize,
            TyKind::Param(param_ty) => Type::GenericArg(param_ty.index),
            TyKind::Alias(_, alias_ty) => Self::from_ty(alias_ty.self_ty(), tyctx),
            TyKind::Closure(def_id, subst) => {
                // this is wrong.
                let kind = ClosureKind::FnOnce;
                let closure = Instance::resolve_closure(tyctx, *def_id, subst, kind)
                    .expect("Could not resolve closure!");
                Self::from_ty(closure.ty(tyctx, ParamEnv::empty()), tyctx)
            }
            TyKind::Array(element, length) => {
                let length = crate::utilis::try_resolve_const_size(length).unwrap();

                let element = Type::from_ty(*element, tyctx);
                DotnetTypeRef::array(element, length).into()
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
pub fn element_type<'tyctx>(src: Ty<'tyctx>) -> Ty<'tyctx> {
    match src.kind() {
        TyKind::Array(element, _) => *element,
        _ => panic!("Can't get element type of {src:?}"),
    }
}
impl From<DotnetTypeRef> for Type {
    fn from(value: DotnetTypeRef) -> Self {
        Self::DotnetType(Box::new(value))
    }
}
const INTEROP_OBJ_TPE_NAME: &str = "RustcCLRInteropManagedClass";
const INTEROP_CHR_TPE_NAME: &str = "RustcCLRInteropManagedChar";
const INTEROP_ARR_TPE_NAME: &str = "RustcCLRInteropManagedArray";
fn is_name_magic(name: &str) -> bool {
    name.contains("RustcCLRInteropManaged")
}
fn magic_type<'tyctx>(
    name: &str,
    adt: &AdtDef<'tyctx>,
    subst: &[GenericArg<'tyctx>],
    ctx: TyCtxt<'tyctx>,
) -> Type {
    match name {
        INTEROP_OBJ_TPE_NAME => {
            if subst.len() != 2 {
                panic!("MAnaged object reference must have exactly 2 generic arguments!");
            }
            let asm = garg_to_string(&subst[0], ctx);
            let asm = Some(asm).filter(|asm| !asm.is_empty());
            let name = garg_to_string(&subst[1], ctx);
            Type::DotnetType(DotnetTypeRef::new(asm.as_ref().map(|x| x.as_str()), &name).into())
        }
        INTEROP_ARR_TPE_NAME => {
            if subst.len() != 2 {
                panic!("Managed array size is not");
            }
            let element = &subst[0].as_type().expect("Arrat type must be specified!");
            let element = Type::from_ty(*element, ctx);
            let dimensions = garag_to_usize(&subst[1],ctx);
            Type::DotnetArray(
                DotnetArray {
                    element,
                    dimensions,
                }
                .into(),
            )
        }
        INTEROP_CHR_TPE_NAME => Type::DotnetChar,
        _ => todo!("Interop type {name:?} is not yet supported!"),
    }
}
fn garag_to_usize<'tyctx>(garg: &GenericArg<'tyctx>, ctx: TyCtxt<'tyctx>) ->u64{
    let usize_const = garg
    .as_const()
    .expect("Generic argument was not an constant!"); 
    if usize_const.ty().is_unit(){
        panic!("Generic argument was not a unit type! ty:{:?}",usize_const.ty());
    }
    else{
        let kind = usize_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let scalar = value.try_to_scalar_int()
                    .expect("String const did not contain valid scalar!");
                scalar. try_to_uint(scalar.size()).unwrap() as u64
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    }
}
fn garg_to_string<'tyctx>(garg: &GenericArg<'tyctx>, ctx: TyCtxt<'tyctx>) -> String {
    let str_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let tpe = str_const
        .ty()
        .builtin_deref(true)
        .expect("Type of generic argument was not a reference, can't resolve as string!");
    if !tpe.ty.is_str() {
        panic!("Generic argument was not a string, but {str_const:?}!");
    } else {
        let kind = str_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let raw_bytes = value
                    .try_to_raw_bytes(ctx, str_const.ty())
                    .expect("String const did not contain valid string!");
                String::from_utf8(raw_bytes.into()).expect("String constant invalid!")
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    }
}
