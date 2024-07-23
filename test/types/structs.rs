#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    improper_ctypes_definitions
)]
#![no_std]
include!("../common.rs");
#[allow(dead_code)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Copy for Vec3 {}
impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}
impl core::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
fn main() {
    let vec3_f32: *mut Vec3 = unsafe { malloc(12) as *mut Vec3 };
    let init_x = black_box(unsafe { (*vec3_f32).x });
    unsafe { (*vec3_f32).x = black_box(128.0) };
    let next_x = black_box(unsafe { (*vec3_f32).x });
    test_eq!(next_x, 128.0);
    test_ne!(next_x, init_x);
    black_box(vec3_f32);
    let a = black_box(Vec3 {
        x: 0.0,
        y: 0.5,
        z: 1.2,
    });
    let b = black_box(Vec3 {
        x: -1.0,
        y: 0.5,
        z: 8.2,
    });
    let c = black_box(a + b);
}
