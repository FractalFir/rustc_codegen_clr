mod atomics;
mod caller_location;
mod ctpop;
mod exact_div;
mod memcmp;
use crate::method::MethodType;
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
                MethodType::Static,
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
                MethodType::Static,
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
        AccessModifer::Public,
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
    //rust_slice(asm);
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
        MethodType::Static,
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
        MethodType::Static,
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
        MethodType::Static,
        FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::U8.into())),
        "__rust_alloc",
        vec![],
    );
    if *crate::config::CHECK_ALLOCATIONS {
        __rust_alloc.set_ops(vec![
            CILOp::LdStr("Allocation of size:".into()),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::console()),
                "Write".into(),
                FnSig::new(&[DotnetTypeRef::string_type().into()], &Type::Void),
                true,
            )),
            CILOp::LDArg(0),
            CILOp::ConvU64(false),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::console()),
                "WriteLine".into(),
                FnSig::new(&[Type::U64], &Type::Void),
                true,
            )),
            CILOp::LDArg(0),
            CILOp::ConvU32(true),
            CILOp::Pop,
            CILOp::LDArg(0),
            CILOp::LDArg(1),
            CILOp::Call(CallSite::boxed(
                native_mem.clone(),
                "AlignedAlloc".into(),
                FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::Void.into())),
                true,
            )),
            CILOp::Ret,
        ]);
    } else {
        __rust_alloc.set_ops(vec![
            CILOp::LDArg(0),
            CILOp::LDArg(1),
            CILOp::Call(CallSite::boxed(
                native_mem.clone(),
                "AlignedAlloc".into(),
                FnSig::new(&[Type::USize, Type::USize], &Type::Ptr(Type::Void.into())),
                true,
            )),
            CILOp::Ret,
        ]);
    }

    asm.add_method(__rust_alloc);
    let mut __rust_dealloc = Method::new(
        AccessModifer::Private,
        MethodType::Static,
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
        MethodType::Static,
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
    //TODO: add volatile prefix to volatile loads
    let mut volatile_load = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::Ptr(Type::U8.into())], &Type::U8),
        "volatile_load",
        vec![],
    );
    volatile_load.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI8, CILOp::Ret]);
    asm.add_method(volatile_load);
    let mut volatile_load = Method::new(
        AccessModifer::Private,
        MethodType::Static,
        FnSig::new(&[Type::Ptr(Type::USize.into())], &Type::USize),
        "volatile_load",
        vec![],
    );
    volatile_load.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndISize, CILOp::Ret]);
    asm.add_method(volatile_load);
    //atomics::add_atomics(asm);
    //ctpop::add_ctpop(asm);
    // exact_div::add_exact_div(asm);
    //memcmp::add_memcmp(asm);
    //memcmp::add_raw_eq(asm);
    add_ptr_offset_from_unsigned(asm);
    //caller_location::add_caller_location(asm,tyctx,&mut TyCache::empty());
    abort(asm);
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
add_method!(
    puts,
    &[Type::Ptr(Box::new(Type::U8))],
    &Type::Void,
    [
        CILOp::Label(0, 0),
        CILOp::LDArg(0),
        CILOp::LDIndI8,
        CILOp::STLoc(0),
        CILOp::LDLoc(0),
        CILOp::LdcI32(0),
        CILOp::BEq(1, 0),
        CILOp::LDLoc(0),
        CILOp::ConvI16(false),
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
        CILOp::ConvUSize(false),
        CILOp::Add,
        CILOp::STArg(0),
        CILOp::GoTo(0, 0),
        CILOp::Label(1, 0),
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
pub fn add_ptr_offset_from_unsigned(asm: &mut Assembly) {
    let ptr_offset_from_unsigned_calls: Box<[_]> = asm
        .call_sites()
        .filter(|call_site| {
            call_site.signature().inputs().len() == 2
                && call_site.name() == "ptr_offset_from_unsigned"
        })
        .cloned()
        .collect();
    for call in ptr_offset_from_unsigned_calls.iter() {
        let rtype: &Type = &call.inputs()[0];
        let mut ptr_offset_from_unsigned = Method::new(
            AccessModifer::Private,
            MethodType::Static,
            call.signature().clone(),
            "raw_eq",
            vec![],
        );
        ptr_offset_from_unsigned.set_ops(match rtype {
            Type::Ptr(inner) => {
                vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::Sub,
                    CILOp::Div,
                    CILOp::SizeOf(inner.clone()),
                    CILOp::Ret,
                ]
            }
            Type::DotnetType(type_ref) => {
                if type_ref.is_valuetype() && type_ref.name_path().contains("PtrComponents") {
                    todo!();
                    /*
                    vec![
                        CILOp::LDArg(0),
                        CILOp::LDArg(1),
                        CILOp::Sub,
                        CILOp::Div,
                        CILOp::SizeOf(inner.clone()),
                        CILOp::Ret,
                    ]*/
                } else {
                    continue;
                }
            }
            _ => continue,
        });
        asm.add_method(ptr_offset_from_unsigned);
    }
}
