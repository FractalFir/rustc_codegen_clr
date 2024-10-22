use crate::{
    fn_ctx::MethodCompileCtx,
    r#type::get_type,
    utilis::{garg_to_string, monomorphize},
};
use cilly::{
    call,
    cil_node::CILNode,
    v2::{cilnode::MethodKind, Assembly, ClassRef, ClassRefIdx, Int, MethodRef},
    Type,
};
use rustc_middle::ty::{AdtDef, ConstKind, GenericArg, ParamEnv, Ty, TyCtxt, TyKind};
/// This struct represetnts either a primitive .NET type (F32,F64), or stores information on how to lookup a more complex type (struct,class,array)
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub struct DotnetArray {
    pub element: Type,
    pub dimensions: u64,
}

#[must_use]
pub fn max_value(tpe: &Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::USize) => {
            let mref = MethodRef::new(
                ClassRef::usize_type(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        Type::Int(Int::U64) => CILNode::V2(asm.alloc_node(u64::MAX)),
        Type::Int(Int::U32) => CILNode::V2(asm.alloc_node(u32::MAX)),
        _ => todo!("Can't get the max value of {tpe:?}"),
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
pub const INTEROP_CLASS_TPE_NAME: &str = "RustcCLRInteropManagedClass";
pub const INTEROP_STRUCT_TPE_NAME: &str = "RustcCLRInteropManagedStruct";
pub const INTEROP_CHR_TPE_NAME: &str = "RustcCLRInteropManagedChar";
pub const INTEROP_ARR_TPE_NAME: &str = "RustcCLRInteropManagedArray";
#[must_use]
/// Checks if a type is a magic interop type.
pub fn is_name_magic(name: &str) -> bool {
    name.contains("RustcCLRInteropManaged")
}
/// Handling of `magic` interop types.
/// # Panics
/// Will panic if interop type is invalid.
#[must_use]
pub fn magic_type<'tcx>(
    name: &str,
    _adt: &AdtDef<'tcx>,
    subst: &[GenericArg<'tcx>],
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Type {
    if name.contains(INTEROP_CLASS_TPE_NAME) {
        assert!(
            subst.len() == 2,
            "Managed object reference must have exactly 2 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx.tcx());
        let assembly = Some(assembly)
            .filter(|assembly| !assembly.is_empty())
            .map(|a| ctx.alloc_string(a));
        let name = garg_to_string(subst[1], ctx.tcx());
        let name = ctx.alloc_string(name);
        let dotnet_tpe = ctx.alloc_class_ref(ClassRef::new(name, assembly, false, [].into()));
        Type::ClassRef(dotnet_tpe)
    } else if name.contains(INTEROP_STRUCT_TPE_NAME) {
        assert!(
            subst.len() == 3,
            "Managed struct reference must have exactly 3 generic arguments!"
        );
        let assembly = garg_to_string(subst[0], ctx.tcx());
        let assembly = Some(assembly)
            .filter(|assembly| !assembly.is_empty())
            .map(|a| ctx.alloc_string(a));
        let name = garg_to_string(subst[1], ctx.tcx());
        let name = ctx.alloc_string(name);
        let dotnet_tpe = ctx.alloc_class_ref(ClassRef::new(name, assembly, true, [].into()));
        Type::ClassRef(dotnet_tpe)
    } else if name.contains(INTEROP_ARR_TPE_NAME) {
        assert!(subst.len() == 2, "Managed array reference must have exactly 2 generic arguments: type and dimension count!");
        let element = &subst[0].as_type().expect("Array type must be specified!");
        let element = ctx.monomorphize(*element);
        let dimensions = garag_to_usize(subst[1], ctx.tcx());
        let element = get_type(element, ctx);

        Type::PlatformArray {
            elem: ctx.alloc_type(element),
            dims: std::num::NonZeroU8::new(dimensions.try_into().unwrap()).unwrap(),
        }
    } else if name.contains(INTEROP_CHR_TPE_NAME) {
        Type::PlatformChar
    } else {
        todo!("Interop type {name:?} is not yet supported!")
    }
}
#[must_use]
pub fn garag_to_usize<'tcx>(garg: GenericArg<'tcx>, _ctx: TyCtxt<'tcx>) -> u64 {
    let usize_const = garg
        .as_const()
        .expect("Generic argument was not an constant!");
    let kind = usize_const.kind();
    match kind {
        ConstKind::Value(ty, value) => {
            let scalar = value
                .try_to_scalar_int()
                .expect("String const did not contain valid scalar!");
            assert!(
                ty.is_integral(),
                "Generic argument was not a unit type! ty:{ty:?}",
            );
            u64::try_from(scalar.to_uint(scalar.size()))
                .expect("Scalar of type usize has value over 2^64")
        }
        _ => todo!("Can't convert generic arg of const kind {kind:?} to string!"),
    }
}
#[must_use]
pub fn tuple_name(elements: &[Type], asm: &Assembly) -> String {
    let generics: String = elements.iter().map(|t| t.mangle(asm)).collect();
    format!(
        "Tuple{generic_count}{generics}",
        generic_count = generics.len()
    )
}
/// Creates a tuple with no more than 8 elements.
#[must_use]
pub fn simple_tuple(elements: &[cilly::v2::Type], asm: &mut Assembly) -> ClassRefIdx {
    // Since no intering can happen at this stage, we can pass an empty AsmStringContainer safely.
    let name = tuple_name(elements, asm);
    let name = asm.alloc_string(name);
    asm.alloc_class_ref(ClassRef::new(name, None, true, [].into()))
}

#[must_use]
pub fn pointer_to_is_fat<'tcx>(
    pointed_type: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: rustc_middle::ty::Instance<'tcx>,
) -> bool {
    is_fat_ptr(
        Ty::new_ptr(tcx, pointed_type, rustc_hir::Mutability::Mut),
        tcx,
        method,
    )
}
#[must_use]
pub fn is_fat_ptr<'tcx>(
    ptr_type: Ty<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: rustc_middle::ty::Instance<'tcx>,
) -> bool {
    use rustc_target::abi::Abi;
    let ptr_type = monomorphize(&method, ptr_type, tcx);
    let layout = tcx
        .layout_of(rustc_middle::ty::ParamEnvAnd {
            param_env: ParamEnv::reveal_all(),
            value: ptr_type,
        })
        .expect("Can't get layout of a type.")
        .layout;
    let abi = layout.abi();
    match abi {
        Abi::Scalar(_) => false,
        Abi::ScalarPair(_, _) => true,
        _ => panic!("Unexpected abi of pointer to {ptr_type:?}. The ABI was:{abi:?}"),
    }
}
