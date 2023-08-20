use crate::{assigment_target::AsigmentValuePosition,Assembly, BaseIR, CLRMethod, VariableType};
use rustc_middle::mir::PlaceElem;
fn projection_element_get(
    element: &PlaceElem,
    var_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (VariableType, BaseIR) {
    match element {
        PlaceElem::Deref => {
            let derefed_type = var_type
                .get_pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.deref_op();
            (derefed_type, deref_op)
        }
        PlaceElem::Field(idx, ty) => {
            let field_type = VariableType::from_ty(*ty);
            let var_name = if let VariableType::Struct(struct_name) = var_type {
                struct_name
            } else {
                panic!("Tried to get a type of a non-struct type {ty:?}.");
            };
            //TODO: figure out how to get the field index PROPELY
            let idx =
                unsafe { std::mem::transmute::<rustc_target::abi::FieldIdx, u32>(*idx) } as usize;
            let getter = asm
                .get_field_getter(idx, &var_name)
                .expect("Can't get field getter!");
            assert_eq!(getter.len(), 1);
            (field_type, getter[0].clone())
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
pub(crate) fn projection_element_set(
    element: &PlaceElem,
    var_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (AsigmentValuePosition,BaseIR) {
    match element {
        PlaceElem::Deref => {
            let derefed_type = var_type
                .get_pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.set_pointed_op();
            (AsigmentValuePosition::AfterAdress,deref_op)
        }
        PlaceElem::Field(idx, ty)=> {
            let field_type = VariableType::from_ty(*ty);
            let var_name = if let VariableType::Struct(struct_name) = var_type {
                struct_name
            } else {
                panic!("Tried to get a type of a non-struct type {ty:?}.");
            };
            //TODO: figure out how to get the field index PROPELY
            let idx =
                unsafe { std::mem::transmute::<rustc_target::abi::FieldIdx, u32>(*idx) } as usize;
            let setter = asm
                .get_field_setter(idx, &var_name)
                .expect("Can't get field getter!");
            assert_eq!(setter.len(), 1);
            (AsigmentValuePosition::BeforeAdress, setter[0].clone())
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
pub(crate) fn projection_get(
    projection: &[PlaceElem],
    local_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (VariableType, Vec<BaseIR>) {
    let mut variable_type = local_type.clone();
    let mut ops = Vec::with_capacity(projection.len());
    for element in projection {
        let (var_type, op) = projection_element_get(element, &variable_type, clr_method, asm);
        ops.push(op);
        variable_type = var_type;
    }
    (variable_type, ops)
}
