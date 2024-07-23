#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    never_type,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports,
    unused_mut,
    private_interfaces,
    non_upper_case_globals,
    unused_unsafe,
    improper_ctypes
)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![no_std]

include!("../common.rs");
fn thread_start(data: *mut ()) {
    unsafe { printf(c"Hello from a launched thread!\n".as_ptr()) };
}

extern "C" {

    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
        >,
        __arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    fn pthread_detach(__th: pthread_t) -> core::ffi::c_int;
    fn pthread_join(__th: pthread_t, res: &mut usize) -> core::ffi::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> core::ffi::c_int;
    fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: size_t,
    ) -> core::ffi::c_int;
    fn exit(_: core::ffi::c_int) -> !;
}
pub type size_t = core::ffi::c_ulong;
pub type pthread_t = core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [core::ffi::c_char; 56],
    pub __align: core::ffi::c_long,
}
#[no_mangle]
pub unsafe extern "C" fn thread1(mut arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    printf(c"Hi from a thread".as_ptr());
    c"Very secret message".as_ptr() as *mut core::ffi::c_void
}
unsafe fn main_0() -> core::ffi::c_int {
    let mut rc: core::ffi::c_int = 0;
    let mut s1: size_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut thid: pthread_t = 0;
    pthread_attr_init(&mut attr);
    s1 = 4096 as core::ffi::c_int as size_t;
    pthread_attr_setstacksize(&mut attr, s1);
    pthread_create(
        &mut thid,
        &mut attr,
        Some(thread1 as unsafe extern "C" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void),
        0 as *mut core::ffi::c_void,
    );

    printf(
        b"Thread ID: %lu\n\0" as *const u8 as *const core::ffi::c_char,
        thid as u64,
    );
    if thid == 0 {
        core::intrinsics::abort();
    }
    let mut res = 64;
    let pt_j = pthread_join(thid, &mut res);
    //let pt_j = pthread_detach(thid);
    printf(c"pt_j %u\n".as_ptr(), pt_j as u32);
    printf(
        c"Recived message \"%s\" from thread.\n".as_ptr(),
        res as *mut u8,
    );
    exit(0 as core::ffi::c_int);
}
pub fn main() {
    unsafe { main_0() };
}
