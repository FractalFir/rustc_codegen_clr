use rustc_middle::mir::PlaceElem;
use crate::{VariableType,BaseIR,CLRMethod,Assembly};
fn projection_element_get(element:&PlaceElem,var_type:&VariableType, clr_method: &CLRMethod,asm:&Assembly)->(VariableType,BaseIR){
    match element{
        PlaceElem::Deref => {
            let derefed_type = var_type.get_pointed_type().expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.deref_op();
            (derefed_type,deref_op)
        },
        PlaceElem::Field(idx, ty) => {
            
            todo!("Can't handle field acces,var_type:{var_type:?} idx:{idx:?}, ty:{ty:?}")
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_)=> todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) =>todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
pub(crate) fn projection_get(projection:&[PlaceElem],local_type:&VariableType, clr_method: &CLRMethod,asm:&Assembly)->Vec<BaseIR>{
    let mut variable_type = local_type.clone();
    let mut ops = Vec::with_capacity(projection.len());
    for element in projection{
        let (var_type,op) = projection_element_get(element,&variable_type,clr_method,asm);
        ops.push(op);
        variable_type = var_type;
    }
    ops
}
