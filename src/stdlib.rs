use crate::{
    cil_op::{CILOp, CallSite},
    method::Method,
    r#type::Type,
    function_sig::FnSig,
    access_modifier::AccessModifer, assembly::Assembly,
};
use crate::r#type::DotnetTypeRef;
macro_rules! add_method {
    ($name:ident,$input:expr,$output:expr,$ops:expr) => {
        fn $name(asm: &mut Assembly) {
            let mut method = Method::new(
                AccessModifer::Private,
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
                FnSig::new($input, $output),
                stringify!($name),
                $locals.into(),
            );
            method.set_ops(($ops).into());
            asm.add_method(method);
        }
    };
}
pub(crate) fn insert_libc(asm: &mut Assembly) {
    // Add core.panic.PanicInfo
    /* 
    asm.add_type(&Type::Struct {
        name: "core.panic.PanicInfo".into(),
        fields: [].into(),
    });
    asm.add_type(&Type::Struct {
        name: "core.panic.PanicInfoUnresolved".into(),
        fields: [].into(),
    });*/
    asm.add_typedef(crate::type_def::TypeDef::nameonly("Unresolved"));
    asm.add_typedef(crate::type_def::TypeDef::nameonly("RustVoid"));
    asm.add_typedef(crate::type_def::TypeDef::nameonly("Foreign"));
    asm.add_typedef(crate::type_def::TypeDef::nameonly("RustStr"));
    let mut rust_slice = crate::type_def::TypeDef::nameonly("RustSlice");
    rust_slice.set_generic_count(1);
    asm.add_typedef(rust_slice);
    math(asm);
    io(asm);
    malloc(asm);
    realloc(asm);
    free(asm);
    abort(asm);
}

fn math(asm: &mut Assembly){
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
             Some(DotnetTypeRef::new(
                Some("System.Console"),
                "System.Console".into()
            )),
            "Write".into(),          
            FnSig::new(
                &[Type::DotnetChar
                ],
                &Type::Void
            ),
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
    [Type::U8]
);
add_method!(
    malloc,
    &[Type::USize],
    &Type::Ptr(Box::new(Type::Void)),
    [
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::new(
               Some("System.Runtime.InteropServices"),
               "System.Runtime.InteropServices.Marshal".into()
           )),
           "AllocHGlobal".into(),          
           FnSig::new(&[Type::ISize], &Type::ISize),
           true,
        )),
        CILOp::Ret,
    ]
);
add_method!(
    free,
    &[Type::Ptr(Box::new(Type::Void))],
    &Type::Void,
    [
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::new(
               Some("System.Runtime.InteropServices"),
               "System.Runtime.InteropServices.Marshal".into()
           )),
           "FreeHGlobal".into(),          
           FnSig::new(&[Type::ISize], &Type::Void),
           true,
        )),
        CILOp::Ret,
    ]
);
add_method!(
    realloc,
    &[Type::Ptr(Box::new(Type::Void)), Type::USize],
    &Type::Ptr(Box::new(Type::Void)),
    [
        CILOp::LDArg(0),
        CILOp::LDArg(1),
        CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::new(
                Some("System.Runtime.InteropServices"),
                "System.Runtime.InteropServices.Marshal"
            )),
            "ReAllocHGlobal".into(),
            FnSig::new(&[Type::ISize, Type::ISize], &Type::ISize),
            true,
        )),
        CILOp::Ret,
    ]
);
//System.Environment.Exit(a_ExitCode)
add_method!(
    abort,
    &[],
    &Type::Void,
    [
        CILOp::LdcI64(0),
        /*
        CILOp::Call(Box::new(CallSite {
            owner: Some(Type::ExternType {
                asm: "System.Runtime".into(),
                name: "System.Environment".into()
            }),
            name: "Exit".into(),
            signature: FnSig::new(&[Type::I32], &Type::Void),
            is_static: true,
        })),*/
        CILOp::ConvISize(false),
        CILOp::LDIndI8,
        CILOp::Pop,
        CILOp::Ret,
    ]
);