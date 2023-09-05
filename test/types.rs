#[no_mangle]
pub extern fn addisize(a:isize,b:isize)->isize{a+b}
//#[no_mangle]
//pub extern fn addi128(a:i128,b:i128)->i128{a+b}
#[no_mangle]
pub extern fn addi64(a:i64,b:i64)->i64{a+b}
#[no_mangle]
pub extern fn addi32(a:i32,b:i32)->i32{a+b}
#[no_mangle]
pub extern fn addi16(a:i16,b:i16)->i16{a+b}
#[no_mangle]
pub extern fn addi8(a:i8,b:i8)->i8{a+b}

#[no_mangle]
pub extern fn addusize(a:usize,b:usize)->usize{a+b}
//#[no_mangle]
//pub extern fn addu128(a:u128,b:u128)->u128{a+b}
#[no_mangle]
pub extern fn addu64(a:u64,b:u64)->u64{a+b}
#[no_mangle]
pub extern fn addu32(a:u32,b:u32)->u32{a+b}
#[no_mangle]
pub extern fn addu16(a:u16,b:u16)->u16{a+b}
#[no_mangle]
pub extern fn addu8(a:u8,b:u8)->u8{a+b}

#[no_mangle]
pub extern fn addf32(a:f32,b:f32)->f32{a+b}
#[no_mangle]
pub extern fn addf64(a:f64,b:f64)->f64{a+b}
#[no_mangle]
pub extern fn boolident(a:bool)->bool{a}
#[no_mangle]
pub extern fn ref_test(a:&u64)->&u64{a}
#[no_mangle]
pub extern fn ref_ident(a:&u64)->&u64{a}
#[no_mangle]
pub extern fn ref_ref_ident<'a,'b>(a:&'a &'b u64)->&'a &'b u64{a}
#[no_mangle]
pub extern fn recive_array(arr:[i32;4]){}
#[no_mangle]
pub extern fn recive_array_ref(arr:&[f64;8]){}
#[no_mangle]
pub extern fn recive_array_array(arr:[[f64;8];8]){}
#[no_mangle]
pub extern fn slice(arr:&[i32]){}
#[no_mangle]
pub extern fn tuple(tup:(i32,i32)){}
pub extern fn init_arr()->[i32;8]{
    [0,1,2,3,4,5,6,7]
}
pub struct GenericType<Inner>(Inner);
/* 
#[no_mangle]
pub extern fn get_option()->GenericType<i32>{
    GenericType(1)
}*/
/*
#[no_mangle]
pub extern fn return_array()->[i32;4]{[0,0,0,0]}

#[no_mangle]
pub extern fn handle_returned_array(){let value = return_array();}
*/
