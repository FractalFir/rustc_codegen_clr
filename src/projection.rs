use crate::{assigment_target::AsigmentValuePosition,Assembly, BaseIR, CLRMethod, VariableType};
use rustc_middle::mir::PlaceElem;
enum Projection<'a,T>{
    OnlyHead(&'a T),
    BodyAndHead(&'a[T],&'a T),
}
/// This function handles any projection elements but the last one. This is needed because some projection elements need to have different behavior when being getting/setting their value. 
/// E.g. Structs fields need to be copied, instead of their adress being taken.
fn projection_element(
    element: &PlaceElem,
    var_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (VariableType, BaseIR) {
    match element {
        PlaceElem::Deref => {
            if let VariableType::Ref(refd) = var_type{
               if let VariableType::Struct(name) = refd.as_ref(){return (VariableType::Struct(name.clone()),BaseIR::Nop)}
            }
            else if let VariableType::RefMut(refd) = var_type{
               if let VariableType::Struct(name) = refd.as_ref(){return (VariableType::Struct(name.clone()),BaseIR::Nop)}
            }
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
            let mut getter = getter[0].clone();
            if let BaseIR::LDField {
                field_parent,
                field_name,
                field_type,
            } = getter{
                getter = BaseIR::LDFieldAdress {field_parent,field_name,field_type};
            }
           
            (field_type, getter)
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
/// This function gets the value of the projected element and should be used on the last element in the projection chain. 
/// It behaves slightly differently than `projection_element` because it, for example copies struct fields instead of just getting their address. 
pub(crate) fn projection_element_get(
    element: &PlaceElem,
    var_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> BaseIR {
    match element {
        PlaceElem::Deref => {
            let derefed_type = var_type
                .get_pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            derefed_type.deref_op()
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
            getter[0].clone()
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
/// This function sest the value of the projected element and should be used on the last element in the projection chain. 
pub(crate) fn projection_element_set(
    element: &PlaceElem,
    var_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (BaseIR) {
    match element {
        PlaceElem::Deref => {
            let derefed_type = var_type
                .get_pointed_type()
                .expect("Dereferenced type is not a pointer, Box  or reference!");
            let deref_op = derefed_type.set_pointed_op();
            deref_op
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
            setter[0].clone()
        }
        PlaceElem::Index(_) => todo!("Can't handle indexing"),
        PlaceElem::Subslice { .. } => todo!("Can't create subslices!"),
        PlaceElem::OpaqueCast(_) => todo!("Can't do opaque casts!"),
        PlaceElem::Downcast(_, _) => todo!("Can't do downcasts!"),
        PlaceElem::ConstantIndex { .. } => todo!("can't handle constant indexing"),
    }
}
fn split_head<'a>(projection:&'a [PlaceElem<'a>])->Projection<'a,PlaceElem<'a>>{
    match projection.len(){
        0=>panic!("ERROR: Can't get the last element of a zero-sized projection chain!"),
        1=>Projection::OnlyHead(&projection[0]),
        _=>Projection::BodyAndHead(&projection[..(projection.len() - 1)],&projection[projection.len() - 1]),
    }
}
pub(crate) fn project<'a,F:Fn(&PlaceElem,&VariableType,&CLRMethod,&Assembly)->BaseIR>(projection:&'a [PlaceElem<'a>], local_type: &VariableType,clr_method: &CLRMethod, asm: &Assembly, head_handler:F)->(Vec<BaseIR>,BaseIR){
    assert!(!projection.is_empty(),"Can't generate ops for empty projection chain!");
    match split_head(projection){
        Projection::OnlyHead(head)=>(Vec::new(),head_handler(head,local_type,clr_method,asm)),
        Projection::BodyAndHead(body,head)=>{
            let mut last_type = local_type.clone();
            let mut ops = Vec::with_capacity(body.len());
            for projection in body{
                let (var_type,op) = projection_element(projection,&last_type,clr_method,asm);
                ops.push(op);
                last_type = var_type;
            }
            let last_op = head_handler(head,&last_type,clr_method,asm);
            (ops,last_op)
        },
    }
}
pub(crate) fn projection_get<'a>(
    projection: &'a [PlaceElem<'a>],
    local_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (Vec<BaseIR>) {
    let (mut addr_calc,getter) = project(projection,local_type,clr_method,asm,projection_element_get);
    addr_calc.push(getter);
    addr_calc
}
/// Used to handle the "body" of a projection chain. Use [`projection_element_get`] or [`projection_element_set`] to handle the head(last element)
pub(crate) fn projection_set<'a>(
    projection: &'a [PlaceElem<'a>],
    local_type: &VariableType,
    clr_method: &CLRMethod,
    asm: &Assembly,
) -> (Vec<BaseIR>,BaseIR) {
    println!("projection:{projection:?}");
    project(projection,local_type,clr_method,asm,projection_element_set)
}
