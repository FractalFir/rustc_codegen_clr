use crate::{
    access_modifier::AccessModifer,
    assembly::Assembly,
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    method::Method,
    r#type::{DotnetTypeRef, Type},
};
/*
pub fn add_ctpop(asm: &mut Assembly) {
    add_ctpop_u64(asm);
}
fn add_ctpop_u64(asm: &mut Assembly) {
    let bit_operations =
        DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
            .with_valuetype(false);
    let bit_operations = Some(bit_operations);
    let mut ctpop = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::U64], &Type::U64),
        "ctpop",
        vec![],
    );
    ctpop.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::Call(CallSite::boxed(
            bit_operations.clone(),
            "PopCount".into(),
            FnSig::new(&[Type::U64], &Type::I32),
            true,
        )),
        CILOp::ConvU64(false),
        CILOp::Ret,
    ]);
    asm.add_method(ctpop);
}
*/