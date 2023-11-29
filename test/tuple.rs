#![allow(improper_ctypes_definitions)]
pub struct Inner{
}
pub struct TupleOwner{
    tuple:(usize,*mut Inner)
}
#[no_mangle]
pub extern "C" fn set_inner(owner:&mut TupleOwner,inner:*mut Inner){
    owner.tuple = (77,inner);
}
pub extern "C" fn read_inner(owner:&TupleOwner)->&*mut Inner{
    &owner.tuple.1
}