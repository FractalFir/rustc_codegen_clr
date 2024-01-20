#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
//use core::intrinsics::{sqrtf32, powif32};

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

fn complex_function1(arg0: i32, arg1: f32, arg2: &mut [i32], arg3: &CustomStruct) -> f32 {
    let mut var0 = (arg0 + 42) as f32 / arg1;
    let mut var1 = 0;

    for &item in arg2.iter() {
        var1 += item;
    }

    if var1 > arg0 {
        var0 += unsafe { core::intrinsics::sqrtf32(var0) } * arg3.field2;
    } else {
        var0 -= unsafe { core::intrinsics::powif32(var0, 2) };
    }

    var0
}

fn complex_function2(arg0: i32, arg1: &CustomEnum, arg2: f32, arg3: &CustomStruct) -> i32 {
    let mut var0 = 0;
    let mut var1 = arg0;
    Put::putnl(var1);
    match arg1 {
        CustomEnum::Variant1(value) => {
            Put::putnl(0);
            var0 += value + arg3.field1;
            Put::putnl(*value);
            Put::putnl(var0);
            var1 *= arg3.custom_method();
        }
        CustomEnum::Variant2(value) => {
            Put::putnl(1);
            var0 -= *value as i32;
            var1 += arg3.field1;
        }
    }
    Put::putnl(var0);
    if var1 % 2 == 0 {
        var0 += var1.pow(2);
    } else {
        var0 -= var1;
    }

    var0
}

fn complex_function3(arg0: i32, arg1: f32, arg2: &mut [i32], arg3: &CustomStruct) -> i32 {
    let mut var0 = arg0;
    let mut var1 = 0;

    for i in (0..arg2.len()).step_by(2) {
        var1 += arg2[i];
    }

    if var1 > 0 {
        var0 *= arg3.field1;
    } else {
        var0 -= arg3.field1;
    }

    var0
}

fn complex_function4(arg0: &mut [f32], arg1: f32, arg2: i32, arg3: &CustomStruct) -> f32 {
    let mut var0 = 0.0;

    for &item in arg0.iter() {
        var0 += item * arg1;
    }

    if arg2 % 2 == 0 {
        var0 *= arg3.custom_method() as f32;
    } else {
        var0 += unsafe { core::intrinsics::sqrtf32(var0) };
    }

    var0
}

fn complex_function5(arg0: CustomStruct, arg1: i32, arg2: &CustomEnum, arg3: &mut [i32]) -> i32 {
    let mut var0 = arg0.field1;

    match arg2 {
        CustomEnum::Variant1(value) => {
            var0 += value * arg1;
        }
        CustomEnum::Variant2(value) => {
            var0 -= *value as i32;
        }
    }

    for &item in arg3.iter() {
        var0 *= item;
    }

    if var0 > arg1 {
        var0 -= arg1.pow(2);
    } else {
        var0 += arg1;
    }

    var0
}

fn complex_function6(arg0: &mut [f32], arg1: CustomStruct, arg2: i32, arg3: &CustomEnum) -> i32 {
    let mut var0 = 0;
    let mut var1 = 1;

    for i in 0..arg0.len() {
        var0 += arg0[i] as i32 * var1;
        var1 *= 2;
    }

    match arg3 {
        CustomEnum::Variant1(value) => {
            var0 += value * arg1.field1;
        }
        CustomEnum::Variant2(value) => {
            var0 -= *value as i32;
        }
    }

    if var0 % arg2 == 0 {
        var0 *= arg1.field1;
    } else {
        var0 += arg1.field1;
    }

    var0
}

fn complex_function7(arg0: f32, arg1: &CustomStruct, arg2: i32, arg3: &CustomEnum) -> f32 {
    let mut var0 = arg0;

    if var0 > 0.0 {
        var0 += unsafe { core::intrinsics::sqrtf32(var0) } * arg1.field2;
    } else {
        var0 -= unsafe { core::intrinsics::powif32(var0, 2) } / arg1.field2;
    }

    let mut var1 = 0;

    match arg3 {
        CustomEnum::Variant1(value) => {
            var1 += value + arg1.field1;
        }
        CustomEnum::Variant2(value) => {
            var1 -= *value as i32;
        }
    }

    while var1 > 0 {
        var0 *= arg2 as f32;
        var1 -= 1;
    }

    var0
}

fn complex_function8(arg0: i32, arg1: &CustomEnum, arg2: f32, arg3: &CustomStruct) -> i32 {
    let mut var0 = arg0;
    let mut var1 = 1;

    for i in 0..5 {
        if i % 2 == 0 {
            var1 *= arg0;
        } else {
            var1 += arg0;
        }
    }

    let mut var2 = 0;

    match arg1 {
        CustomEnum::Variant1(value) => {
            var2 += value * arg0;
        }
        CustomEnum::Variant2(value) => {
            var2 -= *value as i32;
        }
    }

    if var1 > var2 {
        var0 += var1;
    } else {
        var0 -= var2;
    }

    let result = complex_function3(var0, arg2, &mut [1, 2, 3], arg3);

    if result % 2 == 0 {
        var0 *= result;
    } else {
        var0 += result;
    }

    var0
}
fn main(){
    let res = complex_function1(black_box(8), 444.8, &mut [574,4554,-35335,433], &CustomStruct { field1: -43, field2: -0.00545463 });
    test_eq!(res,0.09977405);
    let res = complex_function1(black_box(786), 44.8, &mut [3,-46564,975,-899867], &CustomStruct { field1: 86495, field2: 39355335.34 });
    test_eq!(res,-323.10748);
    let res = complex_function2(black_box(22),&CustomEnum::Variant1(118),-3.14159,&CustomStruct { field1: -942, field2: 0.577567572 });
    Put::putnl(res);
    test_eq!(res,1717935880);
}
