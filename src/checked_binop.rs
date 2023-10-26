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
    match tpe {
        Type::I8 => checked_sadd_type(Type::I32, CILOp::ConvI8(false),CILOp::LdcI32(1<<7)),
        Type::U8 => checked_uadd_type(Type::U8, CILOp::ConvU8(false)),
        Type::I16 => checked_sadd_type(Type::I32, CILOp::ConvI16(false),CILOp::LdcI32(1<<15)),
        Type::U16 => checked_uadd_type(Type::U16, CILOp::ConvU16(false)),
        Type::I32 => checked_sadd_type(Type::I32, CILOp::Nop,CILOp::LdcI32(1<<31)),
        Type::U32 => checked_uadd_type(Type::U32, CILOp::Nop),
        Type::I64 => checked_sadd_type(Type::I32, CILOp::Nop,CILOp::LdcI64(1<<63)),
        Type::U64 => checked_uadd_type(Type::U64, CILOp::Nop),
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
fn checked_uadd_type(tpe: Type, truncate: CILOp) -> Vec<CILOp> {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty = tuple.clone().into();
    vec![
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::Add,
        truncate,
        CILOp::Dup,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::LoadUnderTMPLocal(2),
        CILOp::Or,
        CILOp::Lt,
        CILOp::NewTMPLocal(Type::Bool.into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(Box::new(tuple_ty)),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::GenericArg(1),
            "Item2".into(),
        )),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::GenericArg(0),
            "Item1".into(),
        )),
        CILOp::LoadTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}
fn checked_sadd_type(tpe: Type, truncate: CILOp,mask:CILOp) -> Vec<CILOp> {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty:Type = tuple.clone().into();
    vec![CILOp::NewTMPLocal(tpe.clone().into()),
    CILOp::SetTMPLocal,
    CILOp::NewTMPLocal(tpe.clone().into()),
    CILOp::SetTMPLocal,
    CILOp::LoadTMPLocal,
    CILOp::LoadUnderTMPLocal(1),
    CILOp::Add,
    truncate,
CILOp::NewTMPLocal(tpe.clone().into()),
CILOp::SetTMPLocal,
CILOp::LoadUnderTMPLocal(1),
mask.clone(),
CILOp::And,
CILOp::Dup,
CILOp::LoadUnderTMPLocal(2),
mask.clone(),
CILOp::And,
CILOp::Eq,
CILOp::NewTMPLocal(tpe.clone().into()),
CILOp::SetTMPLocal,
CILOp::LoadUnderTMPLocal(1),
mask.clone(),
CILOp::And,  
CILOp::Eq,
CILOp::Not,
CILOp::LoadTMPLocal,
CILOp::And,
CILOp::NewTMPLocal(tpe.clone().into()),
CILOp::SetTMPLocal,
CILOp::NewTMPLocal(tuple_ty.into()),
CILOp::LoadAddresOfTMPLocal,
CILOp::LoadUnderTMPLocal(1),
CILOp::STField(FieldDescriptor::boxed(
    tuple.clone(),
    Type::GenericArg(0),
    "Item1".into(),
)),
CILOp::LoadAddresOfTMPLocal,
CILOp::LoadUnderTMPLocal(3),
CILOp::STField(FieldDescriptor::boxed(
    tuple.clone(),
    Type::GenericArg(1),
    "Item2".into(),
)),
CILOp::LoadTMPLocal,
CILOp::FreeTMPLocal,
CILOp::FreeTMPLocal,
CILOp::FreeTMPLocal,
CILOp::FreeTMPLocal,
CILOp::FreeTMPLocal,
CILOp::FreeTMPLocal,
    ]
}
fn sub(tpe: Type) -> &'static [CILOp] {
    match tpe {
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
#[test]
fn unsigned_add(){
    //u8
    for a in 0..u8::MAX{
        for b in 0..u8::MAX{
            let added = u8::checked_add(a,b);
            let alg_added = {
                let sum = u8::wrapping_add(a, b);
                if sum < a | b{
                    None
                }
                else{
                    Some(sum)
                }
            };
            assert_eq!(added,alg_added,"checked {a} + {b}");
        }
    }
}
#[test]
fn signed_add(){
    //u8
    for a in i8::MIN..i8::MAX{
        for b in i8::MIN..i8::MAX{
            let added = i8::checked_add(a,b);
            let alg_added = {
                let sum = i8::wrapping_add(a, b);
                let sign_a = a & (0b1000_0000_u8 as i8);
                let sign_b = b & (0b1000_0000_u8 as i8);
                let sum_sign = sum & (0b1000_0000_u8 as i8);
                let signs_equal = sign_a == sign_b;
                if signs_equal && sum_sign != sign_a{
                    None
                }
                else{
                    Some(sum)
                }
            };
            assert_eq!(added,alg_added,"checked {a} + {b}");
        }
    }
}
// Signed int add

//a,b               []                  NewTMPLoc
//a,b               [u]                 CILOp::SetTMPLocal
//a                 [b]                 NewTMPLoc
//a                 [u]                 CILOp::SetTMPLocal
//                  [b,a]               CILOp::LoadTMPLocal
//a                 [b,a]               CILOp::LoadUnderTMPLocal(1)
//a,b               [b,a]               Add
//a+b               [b,a]               NewTMPLoc
//a+b               [b,a,u]             CILOp::SetTMPLocal
//                  [b,a,s]             CILOp::LoadUnderTMPLocal(1)
//a                 [b,a,s]             LDConst(SIGN_MASK)
//a,m               [b,a,s]             And
//am                [b,a,s,am]          Dup
//am,am             [b,a,s,am]          CILOp::LoadUnderTMPLocal(2)
//am,am,b           [b,a,s]             LDConst(SIGN_MASK)
//am,am,b,m         [b,a,s]             And
//am,am,bm          [b,a,s]             Eq
//am,am==bm         [b,a,s]             NewTMPLoc
//am,am==bm         [b,a,s,u]           STMPLoc
//am,               [b,a,s,eq]          CILOp::LoadUnderTMPLocal(1)
//am,s              [b,a,s,eq]          LDConst(SIGN_MASK)
//am,s,m            [b,a,s,eq]          And  
//am,sm             [b,a,s,eq]          Eq 
//am==sm            [b,a,s,eq]          Not 
//am!=sm            [b,a,s,eq]          CILOp::LoadTMPLocal
//am!=sm,am==bm     [b,a,s,eq]          And
//(am!=sm&&am==bm)  [b,a,s,eq]          NewTMPLoc
//(am!=sm&&am==bm)  [b,a,s,eq,u]        CILOp::SetTMPLocal
//                  [b,a,s,eq,cond]     NewTMPLoc
//                  [b,a,s,eq,cond,res] CILOp::LoadTMPLocalA
//res               [b,a,s,eq,cond,res] CILOp::LoadUnderTMPLocal(1)
//res,cond          [b,a,s,eq,cond,res] STFLD(cond)
//                  [b,a,s,eq,cond,res] CILOp::LoadTMPLocalA
//res               [b,a,s,eq,cond,res] CILOp::LoadUnderTMPLocal(3)
//res,s             [b,a,s,eq,cond,res] STFLD(sum)
//                  [b,a,s,eq,cond,res] CILOp::LoadTMPLocal(sum)
//res               [b,a,s,eq,cond,res] FreeTMP
//res               [b,a,s,eq,cond]     FreeTMP
//res               [b,a,s,eq]          FreeTMP
//res               [b,a,s]             FreeTMP
//res               [b,a]               FreeTMP
//res               [b]                 FreeTMP
//res               []