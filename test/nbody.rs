#[derive(Copy,Clone,Default)]
struct Vector3{
    x:f32,
    y:f32,
    z:f32,
}
#[derive(Copy,Clone,Default)]
pub struct AstronomicalBody{
    position:Vector3,
    velocity:Vector3,
    mass:f32
}
#[no_mangle]
pub extern fn init_10body()->[AstronomicalBody;10]{
    let boides = [AstronomicalBody::default();10];
    //let test = Some(0);
    let mut body_idx = 0;
    let nbody_len = boides.len();
    while body_idx < nbody_len{
        body_idx += 1;
    }
    boides 
}
