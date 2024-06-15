use crate::{operand::handle_operand, place::place_set, r#type::tycache::TyCache};
use cilly::{cil_node::CILNode, cil_root::CILRoot, lt_un, Type};

use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, TyCtxt},
};
use rustc_span::source_map::Spanned;
pub fn saturating_add<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    let a = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    let b = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
    let a_type = type_cache.type_from_cache(
        crate::utilis::monomorphize(
            &method_instance,
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
            tyctx,
        ),
        tyctx,
        method_instance,
    );
    let calc = match a_type {
        Type::USize | Type::U64 | Type::U32 | Type::U16 | Type::U8 => {
            let sum = a.clone() + b.clone();
            let or = a | b;
            let flag = lt_un!(sum.clone(), or);
            let max = crate::r#type::max_value(&a_type);
            CILNode::select(a_type, max, sum, flag)
        }
        _ => todo!("Can't use the intrinsic `saturating_add` on {a_type:?}"),
    };
    place_set(destination, tyctx, calc, body, method_instance, type_cache)
}
pub fn saturating_sub<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
    let a = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    let b = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
    let a_ty = crate::utilis::monomorphize(
        &method_instance,
        call_instance.args[0]
            .as_type()
            .expect("saturating_sub works only on types!"),
        tyctx,
    );
    let a_type = type_cache.type_from_cache(a_ty, tyctx, method_instance);
    let calc = match a_type {
        Type::U128 | Type::U64 | Type::U32 | Type::U16 | Type::U8 | Type::USize => {
            let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone());
            let diff =
                crate::binop::sub_unchecked(a_ty, a_ty, tyctx, &method_instance, type_cache, a, b);
            let max = crate::binop::checked::zero(a_ty);
            CILNode::select(a_type, max, diff, undeflow)
        }
        _ => todo!("Can't use the intrinsic `saturating_sub` on {a_type:?}"),
    };
    place_set(destination, tyctx, calc, body, method_instance, type_cache)
}
