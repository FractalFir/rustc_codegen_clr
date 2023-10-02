#[no_mangle]
pub extern fn assign_i8(place: &mut i8, value: &i8) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_i16(place: &mut i16, value: &i16) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_i32(place: &mut i32, value: &i32) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_i64(place: &mut i64, value: &i64) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_isize(place: &mut isize, value: &isize) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_u8(place: &mut u8, value: &u8) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_u16(place: &mut u16, value: &u16) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_u32(place: &mut u32, value: &u32) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_u64(place: &mut u64, value: &u64) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_usize(place: &mut usize, value: &usize) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_f32(place: &mut f32, value: &f32) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_f64(place: &mut f64, value: &f64) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_bool(place: &mut bool, value: &bool) {
    *place = *value;
}

#[no_mangle]
pub extern fn assign_char(place: &mut char, value: &char) {
    *place = *value;
}
