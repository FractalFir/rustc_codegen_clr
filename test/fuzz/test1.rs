#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,unused_assignments,unused_imports)]
#![no_std]
include!("../common.rs");
use core::intrinsics::{cttz, ctlz, maxnumf32, minnumf32, ceilf32, floorf32};

struct CustomStruct {
    field1: i32,
    field2: f32,
}

enum CustomEnum {
    Variant1(i32),
    Variant2(f32),
}

trait CustomTrait {
    fn custom_method(&self) -> i32;
}

impl CustomTrait for CustomStruct {
    fn custom_method(&self) -> i32 {
        self.field1 + self.field1
    }
}

fn function1(arg0: i32, arg1: f32, arg2: &mut [i32], arg3: &CustomStruct) -> f32 {
    let mut result = 0.0;
    let mut sum = 0;

    for &item in arg2.iter() {
        sum += item;
    }

    if sum > arg0 {
        result = maxnumf32(arg1, arg3.field2);
    } else if sum < arg0 {
        result = minnumf32(arg1, arg3.field2);
    } else {
        result = unsafe{ceilf32(arg1) * floorf32(arg3.field2)};
    }

    result
}
fn function2(arg0: i32, arg1: f32, arg2: &mut [i32], arg3: &CustomStruct) -> i32 {
    let mut result = 0;
    let mut sum = 0;

    for &item in arg2.iter() {
        sum += item;
    }
    Put::putnl(sum);
    match arg3.field1 {
        0 => {
            result = cttz(arg0) as i32;
            Put::putnl(result);
        }
        1 => {
            result = ctlz(arg0) as i32;
        }
        _ => {
            result = cttz(sum as u32) as i32 + ctlz(arg0) as i32;
        }
    }
    Put::putnl(result);
    result
}
fn main(){
    test_eq!(3.1415927,function1(643,435333.534,&mut [4,3,4,5,6,7,8,8,9,0,0],&CustomStruct{field1:54334,field2:3.14159265358979323846264338327950288_f32}));
    test_eq!(435333.53,function1(111,435333.534,&mut [4,3,4,5,6,7,8,8,964,43],&CustomStruct{field1:54334,field2:3.14159265358979323846264338327950288_f32}));
    test_eq!(2,cttz(1244 as u32));
    test_eq!(23,function2(643,435333.534,&mut [4,3,4,5,6,7,8,8,9,0,0],&CustomStruct{field1:54334,field2:3.14159265358979323846264338327950288_f32}));
    test_eq!(32,function2(0,4333.534,&mut [4,6,7,8,8,9,0,0],&CustomStruct{field1:0,field2:354.1458979323846264338327950288_f32}));
    test_eq!(32,function2(1,4333.534,&mut [4,6,7,8,5],&CustomStruct{field1:534,field2:354.1458979323846264338327950288_f32}));
}
