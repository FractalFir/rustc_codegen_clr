
use crate::clr_method::LocalPlacement;

use crate::statement::CodegenCtx;
use crate::{types::Type, BaseIR};
use rustc_middle::mir::{Place, PlaceElem};
enum Projection<'a, T> {
    OnlyHead(&'a T),
    BodyAndHead(&'a [T], &'a T),
}
/// This function handles any projection elements but the last one. This is needed because some projection elements need to have different behavior when being getting/setting their value.
/// E.g. Structs fields need to be copied, instead of their adress being taken.
fn projection_element<'ctx>(
    element: &PlaceElem<'ctx>,
    var_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> (Type, Vec<BaseIR>) {
    match element {
        PlaceElem::Deref => {
            todo!("Can't dereference pointers yet!")
        }
        PlaceElem::Field(idx, ty) => {
            todo!("Can't get adresses of fields yet!")
        }
        PlaceElem::Index(index_operand) => {
            todo!("Can't index yet!")
        }
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => {
            todo!("Can't do constant indexing yet!")
        }
    }
}
pub(crate) fn projection_adress<'ctx>(
    place: &Place<'ctx>,
    local_type: &Type,
    codegen_ctx: &CodegenCtx<'ctx, '_>,
) -> Vec<BaseIR> {
    let projection = place.projection;
    let mut ops = Vec::new();
    let mut var_type = local_type.clone();
    ops.push(
        match codegen_ctx.meth().local_id_placement(place.local.into()) {
            LocalPlacement::Arg(arg_id) => BaseIR::LDArg(arg_id),
            LocalPlacement::Var(var_id) => BaseIR::LDLoc(var_id),
        },
    );
    for projection in projection {
        let (vtype, op) = projection_element(&projection, &var_type, codegen_ctx);
        ops.extend(op);
        var_type = vtype;
    }
    ops
}
