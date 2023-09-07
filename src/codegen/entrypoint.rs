use crate::{
    base_ir::{BaseIR, CallSite},
    clr_method::CLRMethod,
    types::Type,
};

pub(crate) fn wrapper(entrypoint: &CallSite) -> CLRMethod {
    if entrypoint.signature.inputs()
        == &[
            Type::ISize,
            Type::Ptr(Box::new(Type::Ptr(Box::new(Type::U8)))),
        ]
        && entrypoint.signature.output() == &Type::ISize
    {
        let sig = crate::FunctionSignature::new(&[], &Type::Void);
        let mut method = CLRMethod::from_raw(
            &[
                BaseIR::LDConstI32(0),
                BaseIR::LDConstI32(0),
                BaseIR::Call(Box::new(entrypoint.clone())),
                BaseIR::Pop,
                BaseIR::Return,
            ],
            &[],
            "entrypoint",
            sig,
        );
        method.add_attribute(crate::clr_method::MethodAttribute::EntryPoint);
        method
    } else {
        panic!("Unsuported entrypoint wrapper signature! entrypoint:{entrypoint:?}");
    }
}
