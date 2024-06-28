#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    improper_ctypes_definitions,
    improper_ctypes
)]

use core::hint::black_box;
pub struct ClassDef;
//include!("../common.rs");
type Object = RustcCLRInteropManagedClass<"System.Runtime", "System.Object">;
type MString = RustcCLRInteropManagedClass<"System.Runtime", "System.String">;
type RustObj_ = RustcCLRInteropManagedClass<"", "RustObj">;
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct RustcCLRInteropManagedClass<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> {
    prevent_construction: usize,
}

pub fn rustc_codegen_clr_new_typedef<
    const NAME: &'static str,
    const IS_VALUETYPE: bool,
    const INHERITS: &'static str,
>() -> ClassDef {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}
pub fn rustc_codegen_clr_finish_type(class: ClassDef) {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}

macro_rules! typedef_fields {
    ($class:ident,$inner:tt) => {};
}
macro_rules! dotnet_typedef {
    () => {};
    (class $name:ident inherits $superclass:path $inner:block) => {
        mod $name {
            #[used]
            static PREVENT_DEAD_CODE_REMOVAL: fn() = rustc_codegen_clr_comptime_entrypoint;
            #[inline(never)]
            pub fn rustc_codegen_clr_comptime_entrypoint() {
                const NAME: &str = stringify!($name);
                const SUPER_CLASS: &str = stringify!($superclass);
                let class = $crate::rustc_codegen_clr_new_typedef::<NAME, false, SUPER_CLASS>();
                typedef_fields!(class, $inner);
                $crate::rustc_codegen_clr_finish_type(class);
            }
        }
    };
    (struct $name:ident inherits $superclass:path $inner:block) => {
        mod $name {
            #[used]
            static PREVENT_DEAD_CODE_REMOVAL: fn() = rustc_codegen_clr_comptime_entrypoint;

            #[inline(never)]
            pub fn rustc_codegen_clr_comptime_entrypoint() {
                const NAME: &str = stringify!($name);
                const SUPER_CLASS: &str = stringify!($superclass);
                let class = $crate::rustc_codegen_clr_new_typedef::<NAME, true, SUPER_CLASS>();
                typedef_fields!(class, $inner);
                $crate::rustc_codegen_clr_finish_type(class);
            }
        }
    };
}
dotnet_typedef! {
    class RustObj inherits System::Runtime::Object{

    }
}
dotnet_typedef! {
    struct RustStruct inherits System::Runtime::Object{

    }
}
fn main() {
    let chr: *mut RustObj_ = core::ptr::null_mut();
    black_box(chr);
}
