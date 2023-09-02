fn test(){}
pub extern "C" fn call_test(){test()}
fn sqr_mag(x:f32,y:f32)->f32{x*x + y*y}
pub extern "C" fn distance(x1:f32,y1:f32,x2:f32,y2:f32)->f32{sqr_mag(x1 - x2,y1 - y2)}
/*
#[no_mangle]
pub extern fn for_loop()->i32{
    for i in 0..100{
    
    }
    0
}*/
