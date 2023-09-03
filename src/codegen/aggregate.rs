use crate::{statement::CodegenCtx, base_ir::BaseIR, types::Type};
use rustc_index::IndexVec;
use rustc_middle::mir::{
    AggregateKind as AKind, Operand, Place as RPlace,
};
use rustc_target::abi::FieldIdx;

use super::place::Place;
fn handle_agregate<'tyctx>(
    codegen_ctx: &CodegenCtx<'tyctx, '_>,
    target_location: &RPlace<'tyctx>,
    aggregate: &AKind<'tyctx>,
    fields: &IndexVec<FieldIdx, Operand<'tyctx>>,
) -> Vec<BaseIR> {
    todo!()
}
enum AggregateKind{
    Tuple,
    Array{
        element_type:Type,
    }
}
impl AggregateKind{
    fn from_ag<'tyctx>(ag:&AKind<'tyctx>,ctx:CodegenCtx<'tyctx, '_>)->Self{
        match ag{
            AKind::Tuple => Self::Tuple,
            AKind::Array(tpe) => Self::Array{element_type:Type::from_ty(tpe, ctx.tyctx())},
            AKind::Generator(_,_,_)=>todo!("Generators are not supported!"),
            AKind::Closure(_,_)=>todo!("Closures are not supported!"),
            AKind::Adt(_,_,_,_,_)=>todo!("Algebraic data types are not yet fully supported"),
        }
    }
}
fn create_aggregate(target_location:Place,aggreagate:AggregateKind,fields:Vec<(u32,Vec<BaseIR>)>)->BaseIR{
    /*match aggreagate{
        AggregateKind::Tuple =>{

        },
    }*/
    todo!() 
}