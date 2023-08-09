pub extern fn identity(a:i32)->i32{a}
pub struct Vector3(f32,f32,f32);
impl std::ops::Add for Vector3{
    type Output = Self;
    fn add(self,rhs:Self)->Self{
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
