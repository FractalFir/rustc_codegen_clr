use crate::{cil::CallSite, IString};
use rustc_middle::middle::exported_symbols::ExportedSymbol;
use rustc_middle::ty::{
    AdtDef, AliasKind, ConstKind, FloatTy, GenericArg, Instance, IntTy, List, ParamEnv, Ty, TyCtxt,
    TyKind, UintTy,
};
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
    // Speical type marking an unresoved type. This is a work around some issues with corelib types. Nothing can ever interact directly with this type.
    Unresolved,
    /// Foregin type. Will never be interacted with directly
    Foreign,
    /// Generic argument
    GenericArg(u32),
    DotnetChar,
    /// Rust FnDefs
    FnDef(IString),
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
    pub fn int_128() -> Self {
        Self::new(Some("System.Runtime"), "System.Int128")
    }
    pub fn uint_128() -> Self {
        Self::new(Some("System.Runtime"), "System.UInt128")
    }
    pub fn usize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.UIntPtr")
    }
    pub fn isize_type() -> Self {
        Self::new(Some("System.Runtime"), "System.IntPtr")
    }
    pub fn new(assembly: Option<&str>, name_path: &str) -> Self {
        Self {
            assembly: assembly.map(std::convert::Into::into),
            name_path: name_path.into(),
            generics: Vec::new(),
            is_valuetype: true,
        }
    }
    pub fn is_valuetype(&self) -> bool {
        self.is_valuetype
    }
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
        self.generics = crate::r#type::ident_gargs(self.generics.len()).into();
    }
    pub fn slice_type(element: Type) -> DotnetTypeRef {
        DotnetTypeRef {
            assembly: None,
            name_path: "RustSlice".into(),
            generics: vec![element],
            is_valuetype: true,
        }
    }
    pub fn slice() -> Self {
        let mut slice_ref = DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents");
        slice_ref.set_generics(vec![Type::USize]);
        slice_ref.into()
    }
    fn generic_from_adt<'ctx>(
        adt_def: &AdtDef<'ctx>,
        subst: &'ctx List<GenericArg<'ctx>>,
        tyctx: TyCtxt<'ctx>,
    ) -> Self {
        let mut generics: Vec<Type> = subst
            .iter()
            .map(|arg| {
                if let Some(resolved) = arg.as_type() {
                    Type::generic_from_ty(resolved, tyctx)
                } else {
                    Type::Unresolved
                }
            })
            .collect();
        for field in adt_def.all_fields() {
            //rustc_middle::ty::List::empty()
            let generic_ty = tyctx.type_of(field.did).instantiate_identity();
            if let TyKind::Alias(_ak, _) = generic_ty.kind() {
                // FIXME: This is wrong, since it pushes the generic type of the child instead of the parrant. Overall - we should traverse trough all
                // ADT locals and take them into account when building the generic list.
                let generic_count = generics.len();
                generics.push(Type::GenericArg(generic_count as u32));
                //panic!("ADT with projection is not supported in generic_from_ty");
            }
        }
        let name = crate::utilis::adt_name(adt_def, tyctx, subst);
        Self {
            assembly: None,
            name_path: name,
            generics,
            is_valuetype: true,
        }
    }
}
impl Type {
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
    pub fn slice_ref(slice_element: Type) -> Type {
        const SLICE_PTR_NAME: &str = "core.ptr.metadata.PtrComponents";

        let mut slice_ref_type = DotnetTypeRef::new(None, SLICE_PTR_NAME);
        let slice_type = DotnetTypeRef::slice_type(slice_element.clone()).into();
        slice_ref_type.set_generics([slice_type, slice_element]);
        slice_ref_type.into()
    }
    pub fn pointer_to(&self) -> Self {
        match self {
            Self::DotnetType(dref) => {
                if dref.assembly.is_none() && dref.name_path() == "RustSlice" {
                    Self::DotnetType(DotnetTypeRef::slice().into())
                } else {
                    Self::Ptr(self.clone().into())
                }
            }
            _ => Self::Ptr(self.clone().into()),
        }
    }
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
    pub fn ref_to(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => todo!("Can't create reference to type {dotnet:?}"),
            _ => Self::Ptr(self.clone().into()),
        }
    }
    pub fn metadata(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => match dotnet.name_path() {
                "PtrComponents" => Type::USize,
                _ => Type::Void,
            },
            _ => Self::Void,
        }
    }
    pub fn as_dotnet(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            _ => None,
        }
    }
    /*
    pub fn from_ty<'ctx>(rust_tpe: Ty<'ctx>, tyctx: TyCtxt<'ctx>, method: &Instance<'ctx>) -> Self {
        if crate::PRINT_TY_CONVERTION {
            println!("ty:{rust_tpe:?}")
        };
        Self::from_ty_kind(rust_tpe.kind(), tyctx, method)
    }*/
    pub fn generic_from_ty<'ctx>(rust_tpe: Ty<'ctx>, tyctx: TyCtxt<'ctx>) -> Self {
        todo!("UJsed generic_from_ty");
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
pub fn element_type(src: Ty<'_>) -> Ty<'_> {
    match src.kind() {
        TyKind::Array(element, _) => *element,
        TyKind::Slice(element) => *element,
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
pub fn is_name_magic(name: &str) -> bool {
    name.contains("RustcCLRInteropManaged")
}
pub fn tuple_type(types: &[Type]) -> DotnetTypeRef {
    if types.len() < 8 {
        let len = types.len();
        let name = format!("System.ValueTuple`{len}");
        crate::r#type::DotnetTypeRef {
            assembly: Some("System.Runtime".into()),
            name_path: name.into(),
            generics: types.into(),
            is_valuetype: true,
        }
    } else {
        panic!("Tuples with more than 8 elements are not supported yet. types:{types:?}");
    }
}
pub fn magic_type<'tyctx>(
    name: &str,
    _adt: &AdtDef<'tyctx>,
    subst: &[GenericArg<'tyctx>],
    ctx: TyCtxt<'tyctx>,
    //method: &Instance<'tyctx>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        if subst.len() != 2 {
            panic!("MAnaged object reference must have exactly 2 generic arguments!");
        }
        let assembly: Box<str> = garg_to_string(&subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(&subst[1], ctx).into();
        let dotnet_tpe = DotnetTypeRef {
            assembly,
            name_path: name,
            generics: vec![],
            is_valuetype: false,
        };
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        if subst.len() != 2 {
            panic!("MAnaged object reference must have exactly 2 generic arguments!");
        }
        let assembly: Box<str> = garg_to_string(&subst[0], ctx).into();
        let assembly = Some(assembly).filter(|assembly| !assembly.is_empty());
        let name = garg_to_string(&subst[1], ctx).into();
        let dotnet_tpe = DotnetTypeRef {
            assembly,
            name_path: name,
            generics: vec![],
            is_valuetype: true,
        };
        Type::DotnetType(dotnet_tpe.into())
    } else if name.contains(INTEROP_ARR_TPE_NAME) {
        if subst.len() != 2 {
            panic!("Managed array size is not");
        }
        let element = &subst[0].as_type().expect("Arrat type must be specified!");
        let element = todo!();
        let dimensions = garag_to_usize(&subst[1], ctx);
        Type::DotnetArray(
            DotnetArray {
                element,
                dimensions,
            }
            .into(),
        )
    } else if name.contains(INTEROP_CHR_TPE_NAME) {
        Type::DotnetChar
    } else {
        todo!("Interop type {name:?} is not yet supported!")
    }
}
fn garag_to_usize<'tyctx>(garg: &GenericArg<'tyctx>, _ctx: TyCtxt<'tyctx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    if !usize_const.ty().is_integral() {
        panic!(
            "Generic argument was not a unit type! ty:{:?}",
            usize_const.ty()
        );
    } else {
        let kind = usize_const.kind();
        match kind {
            ConstKind::Value(value) => {
                let scalar = value
                    .try_to_scalar_int()
                    .expect("String const did not contain valid scalar!");
                scalar.try_to_uint(scalar.size()).unwrap() as u64
            }
            _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
        }
    }
}
/// Creates a tuple with no more than 8 elements.
pub fn simple_tuple(elements: &[Type]) -> DotnetTypeRef {
    assert!(elements.len() <= 8,"Tuple ({elements:?}) contains more than 8 elements, so it can't be stored inside a simple tuple.");
    let name = format!(
        "System.ValueTuple`{element_count}",
        element_count = elements.len()
    );
    let mut dotnet = DotnetTypeRef::new(Some("System.Runtime"), &name);
    dotnet.set_generics(elements);
    dotnet
}
use crate::utilis::garg_to_string;
