use crate::{r#type::Type, method::Method, assembly::Assembly, function_sig::FnSig, access_modifier::AccessModifer, cil::CILOp};

pub fn add_exact_div(asm: &mut Assembly){
    add_exact_div_u64(asm);
    add_exact_div_usize(asm);
}
fn add_exact_div_u64(asm: &mut Assembly){
    let mut exact_div = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::U64,Type::U64], &Type::U64),
        "exact_div",
        vec![],
    );
    exact_div.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::LDArg(1),
        CILOp::Div,
        CILOp::Ret,
    ]);
    asm.add_method(exact_div);
}
fn add_exact_div_usize(asm: &mut Assembly){
    let mut exact_div = Method::new(
        AccessModifer::Private,
        true,
        FnSig::new(&[Type::USize,Type::USize], &Type::USize,),
        "exact_div",
        vec![],
    );
    exact_div.set_ops(vec![
        CILOp::LDArg(0),
        CILOp::LDArg(1),
        CILOp::Div,
        CILOp::Ret,
    ]);
    asm.add_method(exact_div);
}