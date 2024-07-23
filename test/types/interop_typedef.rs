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
    improper_ctypes_definitions,
    improper_ctypes
)]

use core::hint::black_box;
pub struct ClassDef {
    prevent_construction: usize,
}
//include!("../common.rs");
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct RustcCLRInteropManagedClass<const ASSEMBLY: &'static str, const CLASS_PATH: &'static str> {
    prevent_construction: usize,
}
type Object = RustcCLRInteropManagedClass<"System.Runtime", "System.Object">;
type MString = RustcCLRInteropManagedClass<"System.Runtime", "System.String">;
type RustObj_ = RustcCLRInteropManagedClass<"", "RustObj">;
type RustObj2_ = RustcCLRInteropManagedClass<"", "RustObj2">;
#[inline(never)]
pub fn rustc_codegen_clr_add_field_def<T, const FNAME: &'static str>(class: ClassDef) -> ClassDef {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}
#[inline(never)]
pub fn rustc_codegen_clr_add_method_def<
    const VIS: &'static str,
    const MODIFIERS: &'static str,
    const FNAME: &'static str,
    FnType,
>(
    class: ClassDef,
    fn_type: FnType,
) -> ClassDef {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}
#[inline(never)]
pub fn rustc_codegen_clr_new_typedef<
    const NAME: &'static str,
    const IS_VALUETYPE: bool,
    const INHERITS_ASM: &'static str,
    const INHERITS: &'static str,
>() -> ClassDef {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}
#[inline(never)]
pub fn rustc_codegen_clr_finish_type(class: ClassDef) {
    black_box(());
    loop {
        if black_box(true) {
            core::intrinsics::abort()
        }
    }
}

macro_rules! typedef_fields {
    ($typedef:ident,)=>{};
    ($typedef:ident, $field_name:ident : $field_type:ty, $($tail:tt)*) => {
        const $field_name:&str= stringify!( $field_name);
        $typedef = $crate::rustc_codegen_clr_add_field_def::<$field_type, $field_name>($typedef);
        typedef_fields!($typedef, $($tail)*)
    };
    ($typedef:ident, virtual fn $fname:ident($($args:tt)*)->$ret:ty{$($inner:tt)*}, $($tail:tt)*) => {
        use super::*;
        mod $fname{
            use super::super::*;
            #[inline(never)]

            pub extern "C" fn rustc_codegen_clr_not_magic ($($args)*)->$ret{
                $($inner)*
            }
        }
        const FNAME:&str = stringify!($fname);
        #[used]
        static KEEP_FN: extern "C" fn ($($args)*)->$ret = $fname::rustc_codegen_clr_not_magic;

        $typedef = $crate::rustc_codegen_clr_add_method_def::<"pub","virtual",FNAME,_>($typedef,$fname::rustc_codegen_clr_not_magic);
        typedef_fields!($typedef, $($tail)*)
    };
}
macro_rules! dotnet_typedef {
    () => {};

    (class $name:ident inherits [$superasm:path] $superclass:path {  $($inner:tt)* }) => {
        mod $name {
            #[used]
            static PREVENT_DEAD_CODE_REMOVAL: fn() = rustc_codegen_clr_comptime_entrypoint;
            #[inline(never)]
            pub fn rustc_codegen_clr_comptime_entrypoint() {
                const NAME: &str = stringify!($name);
                const SUPER_CLASS: &str = stringify!($superclass);
                const SUPER_ASM: &str = stringify!($superasm);
                let mut class =
                    $crate::rustc_codegen_clr_new_typedef::<NAME, false, SUPER_ASM, SUPER_CLASS>();
                typedef_fields!(class,   $($inner)*);
                $crate::rustc_codegen_clr_finish_type(class);
            }
        }
    };

    (struct $name:ident inherits [$superasm:path]  $superclass:path { $($inner:tt)* }) => {
        mod $name {
            #[used]
            static PREVENT_DEAD_CODE_REMOVAL: fn() = rustc_codegen_clr_comptime_entrypoint;

            #[inline(never)]
            pub fn rustc_codegen_clr_comptime_entrypoint() {
                const NAME: &str = stringify!($name);
                const SUPER_CLASS: &str = stringify!($superclass);
                const SUPER_ASM: &str = stringify!($superasm);
                let class =
                    $crate::rustc_codegen_clr_new_typedef::<NAME, true, SUPER_ASM, SUPER_CLASS>();
                //typedef_fields!(class,  $($inner:tt)*);
                $crate::rustc_codegen_clr_finish_type(class);
            }
        }
    };
}

dotnet_typedef! {
    class RustObj inherits [System::Runtime]System::Runtime::Object{
        a : f32,
        virtual fn ToString(this:RustObj_)->MString{
            panic!()
        },

    }
}

dotnet_typedef! {
    class RustObj2 inherits [System::Runtime]System::Runtime::Object{
        a : f32,
        virtual fn ToString(this:RustObj2_)->MString{
            panic!()
        },

    }
}
/*dotnet_typedef! {
    class RustObj2 inherits RustObj{

    }
}*/
dotnet_typedef! {
    struct RustStruct inherits [System::Runtime]System::Runtime::ValueType{

    }
}
fn main() {
    let chr: *mut RustObj_ = core::ptr::null_mut();
    black_box(chr);
}
