use crate::r#type::DotnetTypeRef;
use crate::{
    access_modifier::AccessModifer,
    assembly::Assembly,
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    method::Method,
    r#type::Type,
};
use rustc_middle::ty::TyCtxt;
macro_rules! add_method {
    ($name:ident,$input:expr,$output:expr,$ops:expr) => {
        fn $name(asm: &mut Assembly) {
            let mut method = Method::new(
                AccessModifer::Private,
                true,
                FnSig::new($input, $output),
                stringify!($name),
                vec![],
            );
            method.set_ops(($ops).to_vec());
            asm.add_method(method);
        }
    };
    ($name:ident,$input:expr,$output:expr,$ops:expr,$locals:expr) => {
        fn $name(asm: &mut Assembly) {
            let mut method = Method::new(
                AccessModifer::Private,
                true,
                FnSig::new($input, $output),
                stringify!($name),
                $locals.into(),
            );
            method.set_ops(($ops).into());
            asm.add_method(method);
        }
    };
}

/// Inserts a small subset of libc and some standard types into an assembly.
pub fn insert_ffi_functions(asm: &mut Assembly, tyctx: TyCtxt) {
    let c_void = Type::c_void(tyctx);
    asm.add_typedef(crate::r#type::TypeDef::new(
        AccessModifer::Private,
        c_void.as_dotnet().unwrap().name_path().into(),
        vec![],
        vec![],
        vec![],
        None,
        0,
        None,
    ));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Unresolved"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustVoid"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Foreign"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustStr"));
    rust_slice(asm);
    math(asm);
    io(asm);
    unlikely(asm);
    //malloc(asm);
    let mut marshal = DotnetTypeRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.Marshal",
    );
    marshal.set_valuetype(false);
    let marshal = Some(marshal);
    let mut malloc = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::USize], &Type::Ptr(c_void.clone().into())),
        "malloc",
        vec![],
    );
    malloc.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            marshal.clone(),
            "AllocHGlobal".into(),
            FnSig::new(&[Type::ISize], &Type::ISize),
            true,
        )),
        CILOp::Ret,
    ]);
    asm.add_method(malloc);
    let mut realloc = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(
            &[Type::Ptr(c_void.clone().into()), Type::USize],
            &Type::Ptr(c_void.clone().into()),
        ),
        "realloc",
        vec![],
    );
    realloc.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::LDArg(1),
        CILOp::Call(CallSite::boxed(
            marshal.clone(),
            "ReAllocHGlobal".into(),
            FnSig::new(&[Type::ISize, Type::ISize], &Type::ISize),
            true,
        )),
        CILOp::Ret,
    ]);
    asm.add_method(realloc);
    let mut native_mem = DotnetTypeRef::new(
        Some("System.Runtime.InteropServices"),
        "System.Runtime.InteropServices.NativeMemory",
    );
    native_mem.set_valuetype(false);
    let native_mem = Some(native_mem);
    let mut __rust_alloc = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::U8.into())),
        "__rust_alloc",
        vec![],
    );
    let msg = CILOp::debug_msg_no_nl("Allocated buffer at adress ");
    __rust_alloc.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::LDArg(1),
        CILOp::Call(CallSite::boxed(
            native_mem.clone(),
            "AlignedAlloc".into(),
            FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::Void.into())),
            true,
        )),
        // Allocation tracing
        /*
        CILOp::Dup,
        msg[0].clone(),
        msg[1].clone(),
        CILOp::ConvU64(false),
        CILOp::new_line(),
        CILOp::debug_u64(),*/
        CILOp::Ret,
    ]);
    asm.add_method(__rust_alloc);
    let mut __rust_dealloc = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(
            &[Type::Ptr(Type::U8.into()), Type::USize, Type::USize],
            &Type::Void,
        ),
        "__rust_dealloc",
        vec![],
    );
    __rust_dealloc.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            native_mem.clone(),
            "AlignedFree".into(),
            FnSig::new(&[Type::Ptr(Type::Void.into())], &Type::Void),
            true,
        )),
        CILOp::Ret,
    ]);
    asm.add_method(__rust_dealloc);
    let mut free = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::Ptr(c_void.clone().into())], &Type::Void),
        "free",
        vec![],
    );
    free.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            marshal.clone(),
            "FreeHGlobal".into(),
            FnSig::new(&[Type::ISize], &Type::Void),
            true,
        )),
        CILOp::Ret,
    ]);
    asm.add_method(free);
    let mut volatile_load = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::Ptr(Type::U8.into())], &Type::U8),
        "volatile_load",
        vec![],
    );
    volatile_load.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI8, CILOp::Ret]);
    asm.add_method(volatile_load);
    abort(asm);
}

fn rust_slice(asm: &mut Assembly) {
    let mut ptr_components = crate::r#type::TypeDef::nameonly("core.ptr.metadata.PtrComponents");
    let mut rust_slice_dotnet = DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents");
    ptr_components.set_generic_count(1);
    rust_slice_dotnet.set_generics([Type::GenericArg(0)]);
    // TODO: constrain this generic to be unmanaged
    ptr_components.add_field("data_address".into(), Type::Ptr(Type::Void.into()));
    ptr_components.add_field("metadata".into(), Type::GenericArg(0));

    asm.add_typedef(ptr_components);
    let mut rust_slice = crate::r#type::TypeDef::nameonly("RustSlice");
    rust_slice.set_generic_count(1);
    asm.add_typedef(rust_slice);
    if asm.types().any(|tpe| tpe.name().contains("PanicInfo")) {
        //rust_begin_unwind(asm);
    }

    //
}

fn math(asm: &mut Assembly) {
    sqrtf32(asm);
}
fn io(asm: &mut Assembly) {
    puts(asm);
}

add_method!(
    sqrtf32,
    &[Type::F32],
    &Type::F32,
    [CILOp::LDArg(0), CILOp::Ret]
);
add_method! {
    rust_begin_unwind,
    &[Type::Ptr(Type::DotnetType(DotnetTypeRef::new(None,"panic.panic_info.PanicInfo").into()).into())],
    &Type::Void,
    CILOp::throw_msg("`rust_begin_unwind` called, but unwinding unsuported!")
}
add_method!(
    puts,
    &[Type::Ptr(Box::new(Type::U8))],
    &Type::Void,
    [
        CILOp::Label(0),
        CILOp::LDArg(0),
        CILOp::LDIndI8,
        CILOp::STLoc(0),
        CILOp::LDLoc(0),
        CILOp::LdcI32(0),
        CILOp::BEq(1),
        CILOp::LDLoc(0),
        CILOp::ConvISize(false),
        CILOp::Call(CallSite::boxed(
            Some(
                DotnetTypeRef::new(Some("System.Console"), "System.Console").with_valuetype(false)
            ),
            "Write".into(),
            FnSig::new(&[Type::DotnetChar], &Type::Void),
            true,
        )),
        CILOp::LDArg(0),
        CILOp::LdcI64(1),
        CILOp::Add,
        CILOp::STArg(0),
        CILOp::GoTo(0),
        CILOp::Label(1),
        CILOp::Ret
    ],
    [(None, Type::U8)]
);
add_method!(
    unlikely,
    &[Type::Bool],
    &Type::Bool,
    [CILOp::LDArg(0), CILOp::Ret,]
);
//System.Environment.Exit(a_ExitCode)
add_method!(abort, &[], &Type::Void, CILOp::throw_msg("Called abort!"));
