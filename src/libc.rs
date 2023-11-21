use crate::r#type::DotnetTypeRef;
use crate::{
    access_modifier::AccessModifer,
    assembly::Assembly,
    cil_op::{CILOp, CallSite, FieldDescriptor},
    function_sig::FnSig,
    method::Method,
    r#type::Type,
};
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
macro_rules! add_tpe_method {
    ($name:ident,$is_static:expr,$input:expr,$output:expr,$ops:expr) => {
        let $name = |tpe: &mut crate::type_def::TypeDef| {
            let mut method = Method::new(
                AccessModifer::Public,
                $is_static,
                FnSig::new($input, $output),
                stringify!($name),
                vec![],
            );
            method.set_ops(($ops).to_vec());
            tpe.add_method(method);
        };
    };
    ($name:ident,$input:expr,$output:expr,$ops:expr,$locals:expr) => {
        let $name = |tpe: &mut crate::type_def::TypeDef| {
            let mut method = Method::new(
                AccessModifer::Public,
                $is_static,
                FnSig::new($input, $output),
                stringify!($name),
                $locals.into(),
            );
            method.set_ops(($ops).into());
            tpe.add_method(method);
        };
    };
}
/// Inserts a small subset of libc and some standard types into an assembly.
pub fn insert_libc(asm: &mut Assembly) {
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
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Unresolved"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustVoid"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("Foreign"));
    asm.add_typedef(crate::r#type::TypeDef::nameonly("RustStr"));
    rust_slice(asm);
    math(asm);
    io(asm);
    malloc(asm);
    realloc(asm);
    free(asm);
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
            Some(DotnetTypeRef::new(Some("System.Console"), "System.Console")),
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
    malloc,
    &[Type::USize],
    &Type::Ptr(Box::new(Type::Void)),
    [
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::new(
                Some("System.Runtime.InteropServices"),
                "System.Runtime.InteropServices.Marshal"
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
                "System.Runtime.InteropServices.Marshal"
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
add_method!(abort, &[], &Type::Void, CILOp::throw_msg("Called abort!"));
/*
add_method!(managed_char_from_utf8,&[Type::U64],&Type::DotnetChar,&[
    CILOp::LDArg(0),
    CILOp::LdcI32(128),
    CILOp::ConvI8(conv.i8),
    CILOp:: And,
    CILOp::Not,
    CILOp::BZero(0),

    CILOp:: LDArg(0),
    CILOp:: LdcI32( 127),
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: conv.u2
    CILOp:: ret

    CILOp:: Lablel(0),
    CILOp:: LDArg(0),
    CILOp:: ldc.i4 224
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: ldc.i4 192
    CILOp:: conv.i8
    CILOp:: bne.un.s IL_0034

    CILOp:: LDArg(0),
    CILOp:: ldc.i4.8
    CILOp:: shr.un
    CILOp:: stloc.0
    CILOp:: LDArg(0),
    CILOp:: LdcI32( 31),
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: ldc.i4.6
    CILOp:: shl
    CILOp:: ldloc.0
    CILOp:: LdcI32( 63
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: or
    CILOp:: conv.u2
    CILOp:: ret

    CILOp::Label(1),
    CILOp:: LDArg(0)
    CILOp:: ldc.i4 240
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: ldc.i4 224
    CILOp:: conv.i8
    CILOp:: bne.un.s IL_0065

    CILOp:: LDArg(0),
    CILOp:: ldc.i4.8
    CILOp:: shr.un
    CILOp:: stloc.1
    CILOp:: LDArg(0),
    CILOp:: LdcI32( 16),
    CILOp:: shr.un
    CILOp:: stloc.2
    CILOp:: LDArg(0),
    CILOp:: LdcI32( 15),
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: LdcI32(12)
    CILOp:: shl
    CILOp:: ldloc.1
    CILOp:: LdcI32(63)
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: ldc.i4.6
    CILOp:: shl
    CILOp:: or
    CILOp:: ldloc.2
    CILOp:: LdcI32(63)
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: or
    CILOp:: conv.u2
    CILOp:: ret

    CILOp:: LDArg(0),
    CILOp:: ldc.i4 248
    CILOp:: conv.i8
    CILOp:: and
    CILOp:: ldc.i4 240
    CILOp:: conv.i8
    CILOp:: bne.un.s IL_0080

    CILOp:: ldstr "UTF8 char requires at least 2 C# codepoints to convert, can't cast!"
    CILOp:: newobj instance void [System.Runtime]System.InvalidCastException::.ctor(string)
    CILOp:: throw

    CILOp:: ldstr "Tried to convert invalid UTF8 char!"
    CILOp:: newobj instance void [System.Runtime]System.ArgumentException::.ctor(string)
    CILOp:: throw
],&[Type::U64,Type::U64,Type::U64]);
*/
// ORIGINAL IL:
/*
.method public static hidebysig
        instance char FromUTF8 (
            uint64 UTF8
        ) cil managed
    {
        // Method begins at RVA 0x2068
        // Code size 139 (0x8b)
        .maxstack 3
        .locals init (
            [0] uint64 byte2,
            [1] uint64 byte2,
            [2] uint64 byte3
        )

        IL_0000: ldarg.0
        IL_0001: ldc.i4 128
        IL_0006: conv.i8
        IL_0007: and
        IL_0008: brtrue.s IL_0011

        IL_000a: ldarg.0
        IL_000b: ldc.i4.s 127
        IL_000d: conv.i8
        IL_000e: and
        IL_000f: conv.u2
        IL_0010: ret

        IL_0011: ldarg.0
        IL_0012: ldc.i4 224
        IL_0017: conv.i8
        IL_0018: and
        IL_0019: ldc.i4 192
        IL_001e: conv.i8
        IL_001f: bne.un.s IL_0034

        IL_0021: ldarg.0
        IL_0022: ldc.i4.8
        IL_0023: shr.un
        IL_0024: stloc.0
        IL_0025: ldarg.0
        IL_0026: ldc.i4.s 31
        IL_0028: conv.i8
        IL_0029: and
        IL_002a: ldc.i4.6
        IL_002b: shl
        IL_002c: ldloc.0
        IL_002d: ldc.i4.s 63
        IL_002f: conv.i8
        IL_0030: and
        IL_0031: or
        IL_0032: conv.u2
        IL_0033: ret

        IL_0034: ldarg.0
        IL_0035: ldc.i4 240
        IL_003a: conv.i8
        IL_003b: and
        IL_003c: ldc.i4 224
        IL_0041: conv.i8
        IL_0042: bne.un.s IL_0065

        IL_0044: ldarg.0
        IL_0045: ldc.i4.8
        IL_0046: shr.un
        IL_0047: stloc.1
        IL_0048: ldarg.0
        IL_0049: ldc.i4.s 16
        IL_004b: shr.un
        IL_004c: stloc.2
        IL_004d: ldarg.0
        IL_004e: ldc.i4.s 15
        IL_0050: conv.i8
        IL_0051: and
        IL_0052: ldc.i4.s 12
        IL_0054: shl
        IL_0055: ldloc.1
        IL_0056: ldc.i4.s 63
        IL_0058: conv.i8
        IL_0059: and
        IL_005a: ldc.i4.6
        IL_005b: shl
        IL_005c: or
        IL_005d: ldloc.2
        IL_005e: ldc.i4.s 63
        IL_0060: conv.i8
        IL_0061: and
        IL_0062: or
        IL_0063: conv.u2
        IL_0064: ret

        IL_0065: ldarg.0
        IL_0066: ldc.i4 248
        IL_006b: conv.i8
        IL_006c: and
        IL_006d: ldc.i4 240
        IL_0072: conv.i8
        IL_0073: bne.un.s IL_0080

        IL_0075: ldstr "UTF8 char requires at least 2 C# codepoints to convert, can't cast!"
        IL_007a: newobj instance void [System.Runtime]System.InvalidCastException::.ctor(string)
        IL_007f: throw

        IL_0080: ldstr "Tried to convert invalid UTF8 char!"
        IL_0085: newobj instance void [System.Runtime]System.ArgumentException::.ctor(string)
        IL_008a: throw
    } // end of method C::FromUTF8
*/
