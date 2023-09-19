#[repr(C)]
pub struct Vector3{
    pub x:f32,
    pub y:f32,
    pub z:f32,
}
//pub extern fn struct_ident(vector:Vector3)->Vector3{vector}

//pub extern fn set_x_1(vector:&mut Vector3){vector.x = 1.0;}

pub extern fn get_x(vector:Vector3)->f32{vector.x}
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