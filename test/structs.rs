#[repr(C)]
pub struct Vector3<Scalar>{
    pub x:Scalar,
    pub y:Scalar,
    pub z:Scalar,
}
//pub extern fn struct_ident(vector:Vector3)->Vector3{vector}

//pub extern fn set_x_1(vector:&mut Vector3){vector.x = 1.0;}

pub extern fn get_x(vector:Vector3<f32>)->f32{vector.x}
pub extern fn get_x_i32(vector:Vector3<i32>)->i32{vector.x}
pub extern fn ref_get_x(vector:&Vector3<f32>)->f32{vector.x}
pub extern fn ref_get_x_i32(vector:&Vector3<i32>)->i32{vector.x}
/* 
impl Vector3{
    pub fn add(&mut self, other:&Self){
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
    pub extern fn sum_x(&mut self, other:&Self){self.x += other.x}
}
*/