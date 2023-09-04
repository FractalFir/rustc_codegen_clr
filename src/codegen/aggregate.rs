use crate::{base_ir::BaseIR, statement::CodegenCtx, types::Type};
use rustc_index::IndexVec;
use rustc_middle::mir::{AggregateKind as AKind, Operand, Place as RustPlace};
use rustc_target::abi::FieldIdx;

use super::place::Place;
pub(crate) fn handle_agregate<'tyctx>(
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &RustPlace<'tyctx>,
    aggregate: &AKind<'tyctx>,
    fields: &IndexVec<FieldIdx, Operand<'tyctx>>,
) -> Vec<BaseIR> {
    let aggregate = AggregateKind::from_ag(aggregate,codegen_ctx);
    let target_location:Place = super::place::Place::from(target_location,codegen_ctx);
    let fields = fields.iter().enumerate().map(|operand|(operand.0 as u32,crate::statement::handle_operand(operand.1, codegen_ctx))).collect();
    create_aggregate(target_location, aggregate, fields)
}
enum AggregateKind {
    Tuple,
    Array { element_type: Type },
}
impl AggregateKind {
    fn from_ag<'tyctx>(ag: &AKind<'tyctx>, ctx: &CodegenCtx<'tyctx, '_>) -> Self {
        match ag {
            AKind::Tuple => Self::Tuple,
            AKind::Array(tpe) => Self::Array {
                element_type: Type::from_ty(tpe, ctx.tyctx()),
            },
            AKind::Generator(_, _, _) => todo!("Generators are not supported!"),
            AKind::Closure(_, _) => todo!("Closures are not supported!"),
            AKind::Adt(_, _, _, _, _) => todo!("Algebraic data types are not yet fully supported"),
        }
    }
}
fn create_aggregate(
    target_location: Place,
    aggreagate: AggregateKind,
    fields: Vec<(u32, Vec<BaseIR>)>,
) -> Vec<BaseIR> {
    match aggreagate{
        AggregateKind::Array{element_type} => todo!("Can't construct array aggreagtes yet!"),
        AggregateKind::Tuple => todo!("Can't construct tuple aggreagtes yet!"),
    }
}
