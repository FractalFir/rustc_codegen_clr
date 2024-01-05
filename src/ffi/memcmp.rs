use crate::{
    access_modifier::AccessModifer,
    assembly::Assembly,
    cil::{CILOp, CallSite},
    function_sig::FnSig,
    method::Method,
    r#type::{DotnetTypeRef, Type},
};

pub fn add_memcmp(asm: &mut Assembly) {
    let sig = FnSig::new(
        &[
            Type::Ptr(Box::new(Type::U8)),
            Type::Ptr(Box::new(Type::U8)),
            Type::USize,
        ],
        &Type::Bool,
    );
    let mut memcmp = Method::new(AccessModifer::Private, true, sig, "memcmp", vec![]);
    memcmp.set_ops(vec![
        CILOp::Label(0),
        // if arg[2] != 0, goto 1, else return 1(true)
        CILOp::LDArg(2),
        CILOp::BTrue(1),
        CILOp::LdcI32(1),
        CILOp::Ret,
        // if *arg[0] == *arg[1], goto 2, else return 0(false)
        CILOp::Label(1),
        CILOp::LDArg(0),
        CILOp::LDIndI8,
        CILOp::LDArg(1),
        CILOp::LDIndI8,
        CILOp::BEq(2),
        CILOp::LdcI32(0),
        CILOp::Ret,
        // arg[0] = arg[0] + 1
        CILOp::Label(2),
        CILOp::LDArg(0),
        CILOp::LdcI32(1),
        CILOp::Add,
        CILOp::STArg(0),
        // arg[1] = arg[1] + 1
        CILOp::LDArg(1),
        CILOp::LdcI32(1),
        CILOp::Add,
        CILOp::STArg(1),
        // arg[2] = arg[2] - 1
        CILOp::LDArg(2),
        CILOp::LdcI32(1),
        CILOp::Sub,
        CILOp::STArg(2),
        CILOp::GoTo(0),
    ]);
    asm.add_method(memcmp);
}
pub fn add_raw_eq(asm: &mut Assembly) {
    let raw_eq_calls: Box<[_]> = asm
        .call_sites()
        .filter(|call_site| {
            call_site.signature().inputs().len() == 2
                && call_site
                    .name()
                    == "raw_eq"
        })
        .cloned()
        .collect();
    for call in raw_eq_calls.iter() {
        let rtype = &call.inputs()[0];
        let mut raw_eq = Method::new(
            AccessModifer::Private,
            true,
            call.signature().clone(),
            "raw_eq",
            vec![],
        );
        raw_eq.set_ops(match rtype{
            Type::Ptr(inner)=>{
                let memcmp_sig = FnSig::new(
                    &[
                        Type::Ptr(Box::new(Type::U8)),
                        Type::Ptr(Box::new(Type::U8)),
                        Type::USize,
                    ],
                    &Type::Bool,
                );
                vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::SizeOf(inner.clone()),
                    CILOp::Call(CallSite::boxed(None,"memcmp".into(),memcmp_sig,true)),
                    CILOp::Ret,
                ]
            }
            _=>continue,
        });
        asm.add_method(raw_eq);
    }
}