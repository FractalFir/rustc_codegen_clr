#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    improper_ctypes_definitions
)]
include!("../common.rs");
#[allow(dead_code)]
#[derive(Copy, Clone, Default)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vector3 {
    fn distance(&self, other: &Self) -> f32 {
        let diff = *self - *other;
        unsafe { sqrtf32(diff.x * diff.x + diff.y * diff.y + diff.z * diff.z) }
    }
}
impl core::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl core::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl core::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
#[derive(Copy, Clone, Default)]
pub struct AstronomicalBody {
    position: Vector3,
    velocity: Vector3,
    mass: f32,
}
impl AstronomicalBody {
    fn distance(&self, other: &Self) -> f32 {
        self.position.distance(&other.position)
    }
    fn handle_interaction(&mut self, other: &Self) {
        let distance = self.distance(other);
        let gravity = ((self.mass + other.mass) / (distance * distance)) / self.mass;
        let dir = self.position - other.position;
        self.velocity = self.velocity + (dir * gravity);
    }
    fn apply_velociy(&mut self) {
        self.position = self.position + self.velocity;
        self.velocity = Vector3::default();
    }
}

struct RNG {
    state: i32,
}
impl RNG {
    fn next_i16(&mut self) -> i16 {
        self.state = 214013 * self.state + 2531011;
        return ((self.state >> 16) & 0xFFFF) as i16;
    }
    fn next_f32(&mut self) -> f32 {
        let fract = self.next_i16() as f32 / 32768.0;
        self.next_i16() as f32 + fract
    }
    fn seeded(seed: i32) -> Self {
        Self { state: seed }
    }
}

#[no_mangle]
pub extern "C" fn init_10body() -> [AstronomicalBody; 4] {
    let mut boides = [
        AstronomicalBody::default(),
        AstronomicalBody::default(),
        AstronomicalBody::default(),
        AstronomicalBody::default(),
    ];
    //let test = Some(0);
    let mut a_body_idx = 0;

    let nbody_len = boides.len();
    let mut rng = RNG::seeded(-85119085);
    while a_body_idx < nbody_len {
        boides[a_body_idx].position.x = rng.next_f32();
        boides[a_body_idx].position.y = rng.next_f32();
        boides[a_body_idx].position.z = rng.next_f32();
        boides[a_body_idx].velocity.x = rng.next_f32() / 1000.0;
        boides[a_body_idx].velocity.y = rng.next_f32() / 1000.0;
        boides[a_body_idx].velocity.z = rng.next_f32() / 1000.0;
        boides[a_body_idx].mass = rng.next_f32() / 100_000.0;
        a_body_idx += 1;
    }
    //println!("DONE!");
    boides
}
#[no_mangle]
pub extern "C" fn tick_10body(boides: &mut [AstronomicalBody; 4], mut tick_count: usize) {
    let mut a_body_idx = 0;
    let mut b_body_idx = 0;
    while 0 < tick_count {
        while a_body_idx < boides.len() {
            let mut a_body = boides[a_body_idx];
            while b_body_idx < boides.len() {
                if b_body_idx == a_body_idx {
                    b_body_idx += 1;
                    continue;
                }
                let b_body = &boides[a_body_idx];
                a_body.handle_interaction(b_body);
                b_body_idx += 1;
            }
            a_body.apply_velociy();
            a_body_idx += 1;
        }
        tick_count -= 1;
    }
}
fn main() {
    let mut tenbody = init_10body();
    tick_10body(&mut tenbody, 25);
    black_box(&tenbody);
}
