use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};

use crate::cil_op::{CILOp, FieldDescriptor};
use crate::r#type::Type;
/// Preforms an checked binary operation.
pub(crate) fn binop_checked<'tcx>(
    binop: BinOp,
    operand_a: &Operand<'tcx>,
    operand_b: &Operand<'tcx>,
    tcx: TyCtxt<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tcx, method, method_instance);
    let ops_b = crate::operand::handle_operand(operand_b, tcx, method, method_instance);
    let ty_a = operand_a.ty(&method.local_decls, tcx);
    let ty_b = operand_b.ty(&method.local_decls, tcx);
    assert_eq!(ty_a, ty_b);
    let ty = Type::from_ty(ty_a, tcx, &method_instance);
    match binop {
        BinOp::Mul | BinOp::MulUnchecked => [ops_a, ops_b, mul(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Add => [ops_a, ops_b, add(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Sub => [ops_a, ops_b, sub(ty).into()]
            .into_iter()
            .flatten()
            .collect(),
        _ => todo!("Can't preform checked op {binop:?}"),
    }
}
fn mul(tpe: Type) -> &'static [CILOp] {
    match tpe {
        _ => todo!("Can't preform checked mul on type {tpe:?} yet!"),
    }
}
fn add(tpe: Type) -> Vec<CILOp> {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(),Type::Bool]);
    let tuple_ty = tuple.clone().into();
    match tpe {
        Type::I32 => vec![
            CILOp::NewTMPLocal(Type::I32.into()),
            CILOp::SetTMPLocal,
            CILOp::NewTMPLocal(Type::I32.into()),
            CILOp::SetTMPLocal,

            CILOp::LoadTMPLocal,
            CILOp::LoadUnderTMPLocal(1),
            CILOp::Add,
            CILOp::Dup,

            CILOp::NewTMPLocal(Type::I32.into()),
            CILOp::SetTMPLocal,
            CILOp::LoadUnderTMPLocal(1),
            CILOp::LoadUnderTMPLocal(2),
            CILOp::Or,
            CILOp::Lt,
            CILOp::NewTMPLocal(Type::I32.into()),
            CILOp::SetTMPLocal,

            CILOp::NewTMPLocal(Box::new(tuple_ty)),
            CILOp::LoadAddresOfTMPLocal,
            CILOp::LoadUnderTMPLocal(1),
            CILOp::STField(FieldDescriptor::boxed(tuple.clone(),Type::GenericArg(1),"Item2".into())),

            CILOp::LoadAddresOfTMPLocal,
            CILOp::LoadUnderTMPLocal(2),
            CILOp::STField(FieldDescriptor::boxed(tuple.clone(),Type::GenericArg(0),"Item1".into())),

            CILOp::LoadTMPLocal,

            CILOp::FreeTMPLocal,
            CILOp::FreeTMPLocal,
            CILOp::FreeTMPLocal,
            CILOp::FreeTMPLocal,
            CILOp::FreeTMPLocal,

        ],
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
fn sub(tpe: Type) -> &'static [CILOp] {
    match tpe {
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
// a b                  []
// a b                  [u]
// a                    [b]
// a                    [b,u]

//                      [b,a]
// a                    [b,a]
// a,b                  [b,a]
// a+b                  [b,a]
// a+b,a+b              [b,a]

// a+b,a+b              [b,a,u]
// a+b                  [b,a,a+b]
// a+b,a                [b,a,a+b]
// a+b,a,b              [b,a,a+b]
// a+b,a|b              [b,a,a+b]
// a+b<a|b              [b,a,a+b,u]
// a+b<a|b              [b,a,a+b,a+b<a|b]

//                      [b,a,a+b ,a+b<a|b,RES]
// *RES                 [b,a,a+b ,a+b<a|b,RES]
// *RES,a+b             [b,a,a+b ,a+b<a|b,RES]
//                      [b,a,a+b ,a+b<a|b,RES]

// *RES                 [b,a,a+b ,a+b<a|b,RES]
// *RES,a+b<a|b         [b,a,a+b,a+b<a|b,RES]
//                      [b,a,a+b,a+b<a|b,RES]

// RES                  [b,a,a+b,a+b<a|b,RES]

// RES                  [b,a,a+b,a+b<a|b]
// RES                  [b,a,a+b]
// RES                  [b,a]
// RES                  [b]
// RES                  []
