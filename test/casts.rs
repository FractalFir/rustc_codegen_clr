#[no_mangle]
pub extern fn cast_f32_to_i32(a:f32)->i32{a as i32}
#[no_mangle]
pub extern fn cast_i8_to_i32(a:i8)->i32{a as i32}
#[no_mangle]
pub extern fn cast_f32_to_i8(a:f32)->i8{a as i8}
#[no_mangle]
pub extern fn cast_i64_to_i32(a:i64)->i32{a as i32}

#[no_mangle]
pub extern fn casti32tof32(a:i32)->f32{a as f32}
