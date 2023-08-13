#[no_mangle]
pub extern fn identity(a:i32)->i32{a}

#[no_mangle]
pub extern fn sqr_mag(ax:i32,ay:i32)->i32{ax*ax + ay*ay}
#[no_mangle]
pub extern fn pow2(power:i32)->i32{1<<power}

#[no_mangle]
pub extern fn addisize(a:isize,b:isize)->isize{a+b}
#[no_mangle]
pub extern fn addi128(a:i128,b:i128)->i128{a+b}
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
#[no_mangle]
pub extern fn addu128(a:u128,b:u128)->u128{a+b}
#[no_mangle]
pub extern fn addu64(a:u64,b:u64)->u64{a+b}
#[no_mangle]
pub extern fn addu32(a:u32,b:u32)->u32{a+b}
#[no_mangle]
pub extern fn addu16(a:u16,b:u16)->u16{a+b}
#[no_mangle]
pub extern fn addu8(a:u8,b:u8)->u8{a+b}

#[no_mangle]
pub extern fn boolident(a:bool)->bool{a}
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
