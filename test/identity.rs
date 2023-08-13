pub extern fn identity(a:i32)->i32{a}
pub extern fn add(a:i32)->i32{a+a}
pub extern fn add2(a:i32,b:i32)->i32{a + b}
pub extern fn sqr_mag(ax:i32,ay:i32)->i32{ax*ax + ay*ay}
pub extern fn pow2(power:i32)->i32{1<<power}
/*
pub extern fn factorial(mut n:i32)->i32{
    let mut factorial = 1;
    while n > 1{
        factorial *= n;
        n-=1;
    }
    //for x in 0..100{}
    factorial
}*/
//pub extern fn call_pow2(power:i32)->i32{ let val = pow2(power); val}
struct Vector3{x:f32,y:f32,z:f32}
/*
impl std::ops::Add for Vector3{
    type Output = Vector3;
    fn add(self,rhs:Self)->Self{
        Self{x:self.x + rhs.x,y:self.y + rhs.y,z:self.z + rhs.z}    
    }
}*/
