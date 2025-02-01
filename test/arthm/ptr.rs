#![feature(ptr_sub_ptr)]
use core::ptr::NonNull; 
fn main(){
	let mut arr:[u8;10] = [0_u8;10];
	let offset = NonNull::new(&mut arr[9] as *mut _).unwrap();
	let start = NonNull::new(&mut arr[0] as *mut _).unwrap();	
	unsafe{assert_eq!(offset.sub_ptr(start),9)};
}
