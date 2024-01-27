#![feature(core_intrinsics,adt_const_params)]
use std::io::Write;
use std::hint::black_box;
extern "C" {
    fn puts(msg: *const u8);
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_call1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1>(arg1:Arg1)->Ret{
    unsafe{puts("Called interop managed call when compiled native code.\n\0".as_ptr())};
    core::intrinsics::abort();
}
macro_rules! test{
    ($name:ident)=>{
        unsafe{puts(concat!("Running test ",stringify!($name),".\n\0").as_ptr())};
        $name();
        unsafe{puts(concat!("Test ",stringify!($name)," succeded.\n\0").as_ptr())};
    }
}
fn collect_test(){
    let numbers:Vec<_> = std::hint::black_box(0..100).collect();
    std::hint::black_box(&numbers);
    for (number,idx) in numbers.iter().enumerate(){
        if std::hint::black_box(number) != *idx{
            unsafe{puts("collect_test failed: items not equal.\n\0".as_ptr())};
            unsafe{core::intrinsics::abort()};
        }
    } 
}
fn map_test(){
    for (number,idx) in std::hint::black_box(0..100).map(|i|{i*i}).enumerate(){
        if std::hint::black_box(number) != idx*idx{
            rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),u64>(number as u64);
            rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),u64>(idx as u64);
            unsafe{puts("map_test1 failed: items not equal.\n\0".as_ptr())};
            unsafe{core::intrinsics::abort()};
        }
    }
    /* 
    let numbers:Vec<_> = std::hint::black_box(0..100).map(|i|{
        unsafe{puts("called map!\n\0".as_ptr())};
        rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),u64>(i as u64);
        i*i
    }).collect();
    std::hint::black_box(&numbers);
    for (number,idx) in numbers.iter().enumerate(){
        if std::hint::black_box(number) != (*idx)*(*idx){
            rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),u64>(number as u64);
            rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),u64>((*idx) as u64);
            unsafe{puts("map_test2 failed: items not equal.\n\0".as_ptr())};
            unsafe{core::intrinsics::abort()};
        }
    } */
}
fn main() {
    let int = std::hint::black_box(8);
    let boxed_int = std::hint::black_box(Box::new(int));
   
    let mut string = String::with_capacity(100);
    string.push('H');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');
    string.push('.');
    string.push('\n');
    string.push('T');
    string.push('h');
    string.push('i');
    string.push('s');
    string.push(' ');
    string.push('m');
    string.push('e');
    string.push('s');
    string.push('s');
    string.push('a');
    string.push('g');
    string.push('e');
    string.push(' ');
    string.push('w');
    string.push('a');
    string.push('s');
    string.push(' ');
    string.push('c');
    string.push('r');
    string.push('e');
    string.push('a');
    string.push('t');
    string.push('e');
    string.push('d');
    string.push(' ');
    string.push('u');
    string.push('s');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push(' ');
    string.push('R');
    string.push('u');
    string.push('s');
    string.push('t');
    string.push('s');
    string.push(' ');
    string.push('`');
    string.push('s');
    string.push('t');
    string.push('d');
    string.push(':');
    string.push(':');
    string.push('s');
    string.push('t');
    string.push('r');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push(':');
    string.push(':');
    string.push('S');
    string.push('t');
    string.push('r');
    string.push('i');
    string.push('n');
    string.push('g');
    string.push('`');
    string.push(' ');
    string.push('t');
    string.push('y');
    string.push('p');
    string.push('e');
    string.push(' ');
    string.push('i');
    string.push('n');
    string.push('s');
    string.push('i');
    string.push('d');
    string.push('e');
    string.push(' ');
    string.push('t');
    string.push('h');
    string.push('e');
    string.push(' ');
    string.push('.');
    string.push('N');
    string.push('E');
    string.push('T');
    string.push(' ');
    string.push('r');
    string.push('u');
    string.push('n');
    string.push('t');
    string.push('i');
    string.push('m');
    string.push('e');
    string.push('!');
    string.push('\n');
    string.push('\0');
    test!(collect_test);
    test!(map_test);
    std::hint::black_box(&string);
    unsafe{puts(string.as_ptr())};
    unsafe{puts("Testing some cool shit\n\0".as_ptr())};
    //let mut f = std::fs::File::create("foo.txt").unwrap();

    //std::hint::black_box(f);
    //std::io::stdout().write_all(b"hello world\n").unwrap();
    let owned = black_box("UWU\n\0").to_owned();
    if owned.len() != 5{
        unsafe{puts(owned.as_ptr())};
        unsafe{core::intrinsics::abort()};
    }
    else{
        unsafe{puts(owned.as_ptr())};
    }
    
    //let s = format!("Hello??? WTF is going on???{}\0",black_box(65));
    //unsafe{puts(s.as_ptr())};
    
    let val = std::hint::black_box(*boxed_int);
    let val = std::hint::black_box(string);
}
