#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn test_fn(a:fn (i32)->i32)->fn (i32)->i32{
    a
}
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn test_fn2(a:i32)->i32{
    a
}
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn return_fn()->fn (i32)->i32{
    fn simple(a:i32)->i32{a}
    simple
}
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn call_simplest_fn_ptr(ptr:fn ()){
    ptr();
}
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn call_fn_ptr_returning_value(ptr:fn ()->i32){
    ptr();
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern fn call_fn_ptr_returning(ptr:fn (i32)->i32){
    ptr(8);
}



