#[no_mangle]
pub extern fn set(reference:&mut i32, value:i32){*reference = value}
#[no_mangle]
pub extern fn set_i2(reference:&mut &mut i32, value:i32){**reference = value}
#[no_mangle]
pub extern fn set_i3(reference:&mut &mut &mut i32, value:i32){***reference = value}
#[no_mangle]
pub extern fn set_i4(reference:&mut &mut &mut &mut i32, value:i32){****reference = value}
#[no_mangle]
pub extern fn get(reference:&mut i32)->i32{*reference}
