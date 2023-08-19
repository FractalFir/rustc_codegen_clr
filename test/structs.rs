pub struct Vector3{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
pub extern fn struct_ident(vector:Vector3)->Vector3{vector}
/*
pub extern fn set_x(mut vector:Vector3)->Vector3{vector.x = 1.0;vector}
*/
pub extern fn get_x(mut vector:Vector3)->f32{vector.x}
