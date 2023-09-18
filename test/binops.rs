#[no_mangle]
pub extern fn subi32(a:i32,b:i32)->i32{a-b}
#[no_mangle]
pub extern fn muli32(a:i32,b:i32)->i32{a*b}
#[no_mangle]
pub extern fn divi32(a:i32,b:i32)->i32{a/b}
#[no_mangle]
pub extern fn remi32(a:i32,b:i32)->i32{a%b}
#[no_mangle]
pub extern fn shli32(a:i32,b:i32)->i32{a<<b}
#[no_mangle]
pub extern fn shri32(a:i32,b:i32)->i32{a>>b}
#[no_mangle]
pub extern fn xori32(a:i32,b:i32)->i32{a^b}
#[no_mangle]
pub extern fn ori32(a:i32,b:i32)->i32{a|b}
#[no_mangle]
pub extern fn andi32(a:i32,b:i32)->i32{a&b}
#[no_mangle]
pub extern fn eq(a:i32,b:i32)->bool{a == b}
#[no_mangle]
pub extern fn neq(a:i32,b:i32)->bool{a != b}
//More complex tests
#[no_mangle]
pub extern fn sqr_mag(ax:i32,ay:i32)->i32{ax*ax + ay*ay}
#[no_mangle]
pub extern fn pow2(power:i32)->i32{1<<power}
//Not binary, but unary.
#[no_mangle]
pub extern fn negate_i32(val:i32)->i32{-val}
#[no_mangle]
pub extern fn negate_f32(val:f32)->f32{-val}

