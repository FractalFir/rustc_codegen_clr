use crate::{
    assembly_exporter::AssemblyExporter,
    base_ir::{BaseIR, CallSite},
    clr_method::CLRMethod,
    types::Type,
    FunctionSignature,
};
macro_rules! add_method {
    ($name:ident,$input:expr,$output:expr,$ops:expr) => {
        fn $name(asm: &mut impl AssemblyExporter) {
            asm.add_method(CLRMethod::from_raw(
                &($ops),
                &[],
                stringify!($name),
                FunctionSignature::new($input, $output),
            ));
        }
    };
    ($name:ident,$input:expr,$output:expr,$ops:expr,$locals:expr) => {
        fn $name(asm: &mut impl AssemblyExporter) {
            asm.add_method(CLRMethod::from_raw(
                &($ops),
                &$locals,
                stringify!($name),
                FunctionSignature::new($input, $output),
            ));
        }
    };
}
pub(crate) fn insert_libc(asm: &mut impl AssemblyExporter) {
    // Add core.panic.PanicInfo
    asm.add_type(&Type::Struct {
        name: "core.panic.PanicInfo".into(),
        fields: [].into(),
    });
    asm.add_type(&Type::Struct {
        name: "core.panic.PanicInfoUnresolved".into(),
        fields: [].into(),
    });
    math(asm);
    io(asm);
    malloc(asm);
    realloc(asm);
    free(asm);
}

fn math(asm: &mut impl AssemblyExporter) {
    sqrtf32(asm);
}
fn io(asm: &mut impl AssemblyExporter) {
    puts(asm);
}
add_method!(
    sqrtf32,
    &[Type::F32],
    &Type::F32,
    [BaseIR::LDArg(0), BaseIR::Return]
);
add_method!(
    puts,
    &[Type::Ptr(Box::new(Type::U8))],
    &Type::Void,
    [
        BaseIR::BBLabel { bb_id: 0 },
        BaseIR::LDArg(0),
        BaseIR::Volatile(Box::new(BaseIR::LDIndIn(1))),
        BaseIR::STLoc(0),
        BaseIR::LDLoc(0),
        BaseIR::LDConstI32(0),
        BaseIR::BEq { target: 1 },
        BaseIR::LDLoc(0),
        BaseIR::ConvI16,
        BaseIR::Call(Box::new(CallSite {
            owner: Some(Type::ExternType {
                asm: "System.Console".into(),
                name: "System.Console".into()
            }),
            is_static: true,
            name: "Write".into(),
            signature: FunctionSignature::new(
                &[Type::ExternType {
                    asm: "".into(),
                    name: "char".into()
                }],
                &Type::Void
            )
        })),
        BaseIR::LDArg(0),
        BaseIR::LDConstI32(1),
        BaseIR::Add,
        BaseIR::STArg(0),
        BaseIR::GoTo { target: 0 },
        BaseIR::BBLabel { bb_id: 1 },
        BaseIR::Return
    ],
    [Type::U8]
);
add_method!(
    malloc,
    &[Type::USize],
    &Type::Ptr(Box::new(Type::Void)),
    [
        BaseIR::LDArg(0),
        BaseIR::Call(Box::new(CallSite {
            owner: Some(Type::ExternType {
                asm: "System.Runtime.InteropServices".into(),
                name: "System.Runtime.InteropServices.Marshal".into()
            }),
            name: "AllocHGlobal".into(),
            signature: FunctionSignature::new(&[Type::ISize], &Type::ISize),
            is_static: true,
        })),
        BaseIR::Return,
    ]
);
add_method!(
    free,
    &[Type::Ptr(Box::new(Type::Void))],
    &Type::Void,
    [
        BaseIR::LDArg(0),
        BaseIR::Call(Box::new(CallSite {
            owner: Some(Type::ExternType {
                asm: "System.Runtime.InteropServices".into(),
                name: "System.Runtime.InteropServices.Marshal".into()
            }),
            name: "FreeHGlobal".into(),
            signature: FunctionSignature::new(&[Type::ISize], &Type::Void),
            is_static: true,
        })),
        BaseIR::Return,
    ]
);
add_method!(
    realloc,
    &[Type::Ptr(Box::new(Type::Void)), Type::USize],
    &Type::Ptr(Box::new(Type::Void)),
    [
        BaseIR::LDArg(0),
        BaseIR::LDArg(1),
        BaseIR::Call(Box::new(CallSite {
            owner: Some(Type::ExternType {
                asm: "System.Runtime.InteropServices".into(),
                name: "System.Runtime.InteropServices.Marshal".into()
            }),
            name: "ReAllocHGlobal".into(),
            signature: FunctionSignature::new(&[Type::ISize, Type::ISize], &Type::ISize),
            is_static: true,
        })),
        BaseIR::Return,
    ]
);
