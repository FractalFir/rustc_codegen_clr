#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
use core::mem::MaybeUninit;

include!("../common.rs");
enum CustomEnum {
    Variant1(i32),
    Variant2(f32),
}

#[derive(Clone, Copy)]
enum Animal {
    Cow(u8),
    Dog(u64),
}
fn test_indirect(animal: &Animal) {
    let animal = black_box(animal);
    match animal {
        Animal::Cow(val) => {
            let v: u8 = val + 1_u8;
            Put::putnl(*val);
            test_eq!(*val, 8);
            test_eq!(v, 9);
        }
        Animal::Dog(val) => {
            Put::putnl(*val);
            test_eq!(0, 1);
        }
    }
}
fn main() {
    let animal = black_box(Animal::Cow(8));
    match animal {
        Animal::Cow(val) => {
            Put::putnl(val);
            test_eq!(val, 8);
        }
        Animal::Dog(val) => {
            Put::putnl(val);
            test_eq!(0, 1);
        }
    }
    test_indirect(&animal);
    test_eq!(complex_function2(&CustomEnum::Variant1(118)), 118);
    let mut stack_buf = AlignedStorage::<u8, 4096>::new();
    let stack_scratch = stack_buf.as_uninit_slice_mut();
    black_box(stack_scratch);
}

fn complex_function2(arg1: &CustomEnum) -> i32 {
    match arg1 {
        CustomEnum::Variant1(value) => {
            let arg_slice = unsafe {
                core::slice::from_raw_parts(
                    arg1 as *const _ as *const u8,
                    core::mem::size_of::<CustomEnum>(),
                )
            };
            for val in arg_slice.iter() {
                Put::putnl(*val);
            }
            let arg_ptr = arg1 as *const _ as *const u8;

            Put::putnl(*value);
            test_eq!(*value, 118);
            *value
        }
        CustomEnum::Variant2(value) => {
            Put::putnl(1);
            //var0 -= *value as i32;
            *value as i32
        }
    }
}
#[no_mangle]
#[inline(never)]
pub fn return_maybeuinint_array() -> [core::mem::MaybeUninit<i32>; 16] {
    [core::mem::MaybeUninit::uninit(); 16]
}
#[repr(C)]
struct AlignedStorage<T, const N: usize> {
    _align: [T; 0],
    storage: [MaybeUninit<u8>; N],
}

impl<T, const N: usize> AlignedStorage<T, N> {
    fn new() -> Self {
        Self {
            _align: [],
            storage: [const { MaybeUninit::uninit() }; N],
        }
    }

    fn as_uninit_slice_mut(&mut self) -> &mut [MaybeUninit<T>] {
        let len = N / core::mem::size_of::<T>();

        // SAFETY: `_align` ensures we are correctly aligned.
        unsafe { core::slice::from_raw_parts_mut(self.storage.as_mut_ptr().cast(), len) }
    }
}
