use crate::IString;
use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::{AdtDef, ConstKind, FloatTy, GenericArg, IntTy, Ty, TyCtxt, TyKind, UintTy};
/// This struct represetnts either a primitive .NET type (F32,F64), or stores information on how to lookup a more complex type (struct,class,array)
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub enum Type {
    /// Void type
    Void,
    /// Boolean type
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
    /// A refernece to a .NET type
    DotnetType(Box<DotnetTypeRef>),
    /// A reference to a .NET array type
    DotnetArray(Box<DotnetArray>),
    // Pointer to a type
    Ptr(Box<Self>),
    /// A managed reference `&`. IS NOT EQUIVALENT TO RUST `&`!
    ManagedReference(Box<Self>),
    // Speical type marking an unresoved type. This is a work around some issues with corelib types. Nothing can ever interact directly with this type.
    Unresolved,
    /// Foregin type. Will never be interacted with directly
    Foreign,
    /// Generic argument
    GenericArg(u32),
    CallGenericArg(u32),
    DotnetChar,
    /// Rust FnDefs
    FnDef(IString),
    DelegatePtr(Box<crate::function_sig::FnSig>),
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
    // In cause of `System.BadImageFormatException: Expected value type but got type kind 14` check if `is_valuetype` is always correct!
    is_valuetype: bool,
}
impl DotnetTypeRef {
    #[must_use]
    pub fn int_128() -> Self {
        Self::new(Some("System.Runtime"), "System.Int128")
    }
    #[must_use]
    pub fn uint_128() -> Self {
        Self::new(Some("System.Runtime"), "System.UInt128")
    }
    #[must_use]
    pub fn usize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.UIntPtr")
    }
    #[must_use]
    pub fn isize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.IntPtr")
    }
    #[must_use]
    pub fn type_handle_type() -> Self {
        Self::new(Some("System.Runtime"), "System.RuntimeTypeHandle")
    }
    #[must_use]
    pub fn type_type() -> Self {
        Self::new(Some("System.Runtime"), "System.Type").with_valuetype(false)
    }
    #[must_use]
    pub fn object_type() -> Self {
        Self::new(Some("System.Runtime"), "System.Object").with_valuetype(false)
    }
    #[must_use]
    pub fn with_valuetype(mut self, valuetype: bool) -> Self {
        self.set_valuetype(valuetype);
        self
    }
    #[must_use]
    pub fn compiler_services_unsafe() -> Self {
        DotnetTypeRef::new(
            Some("System.Runtime"),
            "System.Runtime.CompilerServices.Unsafe",
        )
        .with_valuetype(false)
    }
    pub fn new(assembly: Option<&str>, name_path: &str) -> Self {
        Self {
            assembly: assembly.map(std::convert::Into::into),
            name_path: name_path.into(),
            generics: Vec::new(),
            is_valuetype: true,
        }
    }
    #[must_use]
    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }
    #[must_use]
    pub fn tpe_prefix(&self) -> &'static str {
        if self.is_valuetype() {
            "valuetype"
        } else {
            "class"
        }
    }
    pub fn set_valuetype(&mut self, is_valuetype: bool) {
        self.is_valuetype = is_valuetype;
    }
    #[must_use]
    pub fn array(element: Type, length: usize) -> Self {
        let name = crate::r#type::type_def::arr_name(length, &element);
        DotnetTypeRef::new(None, &name)
    }
    pub fn append_path(&mut self, append: &str) {
        let mut name_path = self.name_path.to_string();
        name_path.push_str(append);
        self.name_path = name_path.into();
    }
    pub fn asm(&self) -> Option<&str> {
        self.assembly.as_ref().map(std::convert::AsRef::as_ref)
    }
    #[must_use]
    pub fn name_path(&self) -> &str {
        self.name_path.as_ref()
    }
    #[must_use]
    pub fn generics(&self) -> &[Type] {
        self.generics.as_ref()
    }
    pub fn set_generics(&mut self, generics: impl Into<Vec<Type>>) {
        self.generics = generics.into();
    }
}
impl Type {
    #[must_use]
    /// Finds the `c_void` type.
    /// # Panics
    /// Will panic if `c_void` is not defined.
    pub fn c_void(tyctx: TyCtxt) -> Type {
        let lang_items = tyctx.lang_items();
        let c_void = lang_items.c_void().expect("c_void not defined.");
        let name = rustc_codegen_ssa::back::symbol_export::symbol_name_for_instance_in_crate(
            tyctx,
            ExportedSymbol::NonGeneric(c_void),
            c_void.krate,
        );
        let name = crate::utilis::escape_class_name(&name);
        DotnetTypeRef::new(None, &name).into()
    }
    #[must_use]
    pub fn map_generic(&self, generics: &[Type]) -> Option<Type> {
        match self {
            Self::GenericArg(arg) => generics.get(*arg as usize).cloned(),
            Self::DotnetType(dref) => {
                let mut dref = dref.clone();
                let dref_generics: Option<Vec<_>> = dref
                    .generics()
                    .iter()
                    .map(|gtype| gtype.map_generic(generics))
                    .collect();
                dref.set_generics(dref_generics?);
                Some(Self::DotnetType(dref))
            }
            _ => Some(self.clone()),
        }
    }
    #[must_use]
    pub fn ref_to(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => todo!("Can't create reference to type {dotnet:?}"),
            _ => Self::Ptr(self.clone().into()),
        }
    }
    #[must_use]
    pub fn metadata(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => match dotnet.name_path() {
                "PtrComponents" => Type::USize,
                _ => Type::Void,
            },
            _ => Self::Void,
        }
    }
    #[must_use]
    pub fn as_dotnet(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            _ => None,
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
/// Gets the element type of a slice OR array.
/// # Panics
/// Panics if type is not a slice or an array.
#[must_use]
pub fn element_type(src: Ty<'_>) -> Ty<'_> {
    match src.kind() {
        TyKind::Slice(element) | TyKind::Array(element, _) => *element,
        _ => panic!("Can't get element type of {src:?}"),
    }
}
impl From<DotnetTypeRef> for Type {
    fn from(value: DotnetTypeRef) -> Self {
        Self::DotnetType(Box::new(value))
    }
}
const INTEROP_CLASS_TPE_NAME: &str = "RustcCLRInteropManagedClass";
const INTEROP_STRUCT_TPE_NAME: &str = "RustcCLRInteropManagedStruct";
const INTEROP_CHR_TPE_NAME: &str = "RustcCLRInteropManagedChar";
const INTEROP_ARR_TPE_NAME: &str = "RustcCLRInteropManagedArray";
#[must_use]
/// Checks if a type is a magic interop type.
pub fn is_name_magic(name: &str) -> bool {
    name.contains("RustcCLRInteropManaged")
}
/// Handling of `magic` interop types.
/// # Panics
/// Will panic if interop type is invalid.
#[must_use]
pub fn magic_type<'tyctx>(
    name: &str,
    _adt: &AdtDef<'tyctx>,
    subst: &[GenericArg<'tyctx>],
    ctx: TyCtxt<'tyctx>,
    //method: &Instance<'tyctx>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed object reference must have exactly 2 generic arguments!"
        );
        let assembly: Box<str> = garg_to_string(subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx).into();
        let dotnet_tpe = DotnetTypeRef {
            assembly,
            name_path: name,
            generics: vec![],
            is_valuetype: false,
        };
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed struct reference must have exactly 2 generic arguments!"
        );
        let assembly: Box<str> = garg_to_string(subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(subst[1], ctx).into();
        let dotnet_tpe = DotnetTypeRef {
            assembly,
            name_path: name,
            generics: vec![],
            is_valuetype: true,
        };
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_ARR_TPE_NAME) {
        assert!(subst.len() == 2, "Managed array reference must have exactly 2 generic arguments: type and dimension count!");
        let element = &subst[0].as_type().expect("Array type must be specified!");
        let dimensions = garag_to_usize(subst[1], ctx);
        let _ = (element, dimensions);
        /*

        Type::DotnetArray(
            DotnetArray {
                element,
                dimensions,
            }
            .into(),
        )*/
        todo!()
    } else if name.contains(INTEROP_CHR_TPE_NAME) {
        Type::DotnetChar
    } else {
        todo!("Interop type {name:?} is not yet supported!")
    }
}
fn garag_to_usize<'tyctx>(garg: GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    if usize_const.ty().is_integral() {
        let kind = usize_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let scalar = value
                    .try_to_scalar_int()
                    .expect("String const did not contain valid scalar!");
                u64::try_from(scalar.try_to_uint(scalar.size()).unwrap())
                    .expect("Scalar of type usize has value over 2^64")
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    } else {
        panic!(
            "Generic argument was not a unit type! ty:{:?}",
            usize_const.ty()
        );
    }
}
/// Creates a tuple with no more than 8 elements.
#[must_use]
pub fn simple_tuple(elements: &[Type]) -> DotnetTypeRef {
    //assert!(elements.len() <= 8,"Tuple ({elements:?}) contains more than 8 elements, so it can't be stored inside a simple tuple.");
    let name = tuple_name(elements);
    let dotnet = DotnetTypeRef::new(None, &name);
    dotnet
}
use crate::utilis::garg_to_string;

use super::tuple_name;
pub fn pointer_to_is_fat<'tyctx>(
    mut pointed_type: Ty<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method: Option<rustc_middle::ty::Instance<'tyctx>>,
) -> bool {
    use rustc_middle::ty::ParamEnv;
    method.inspect(|method| {
        pointed_type = crate::utilis::monomorphize(method, pointed_type, tyctx);
    });
    let (metadata, fat_if_not_sized) = pointed_type.ptr_metadata_ty(tyctx, |mut ty| {
        method.inspect(|method| {
            ty = crate::utilis::monomorphize(method, ty, tyctx);
        });
        ty
    });
    //TODO: fat_if_not_sized is suposed to tell me if a pointer being fat depends on if the type is sized.
    // I am not sure how this is suposed to work exactly, so it gets ignored for now.
    //let is_sized = pointed_type.is_sized(tyctx, ParamEnv::reveal_all());
    let is_trivialy_sized = pointed_type.is_trivially_sized(tyctx);
    if is_trivialy_sized {
        // Sized types don't need fat pointers
        false
    } else {
        // TODO: PROPELY check if type is sized
        !pointed_type.is_sized(tyctx, ParamEnv::reveal_all())
        //true
    }
}
