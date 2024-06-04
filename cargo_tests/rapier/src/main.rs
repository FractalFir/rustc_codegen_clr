#![feature(fmt_internals,sync_unsafe_cell,numfmt,core_intrinsics,flt2dec,no_sanitize,extern_types,specialization,maybe_uninit_uninit_array,maybe_uninit_slice,never_type,exposed_provenance)]
#![feature(adt_const_params,test)]
use std::fmt::Debug;

//mod fmt;
mod term;
use std::fs::File;
use std::hint;
use std::io::{self, Write, Read};
use std::net::TcpStream;

use term::terminfo::searcher::get_dbpath_for_term;
fn main() {
    let strings:Vec<String> = vec!["Hi".into(),"Bob".into(),"John".into()];
    drop(std::hint::black_box(strings));
    
    eprintln!("Formatting in .NET! Test int: {int} Test float:{float}
    dur:{dur:?}",int = std::hint::black_box(64),float = std::hint::black_box(3.14159),dur = std::hint::black_box(std::time::Duration::from_millis(1000)));

    let five: std::rc::Rc<std::cell::UnsafeCell<i32>> = std::rc::Rc::new(std::cell::UnsafeCell::new(5));

    std::hint::black_box(five.clone());
    //net_main().unwrap();
    get_dbpath_for_term("xterm-256color");
    /*println!("preparing to create term.");
    let term = term::terminfo::TermInfo::from_env();
    println!("Created term.");
    println!("{term:?}");*/
}
fn net_main() -> io::Result<()> {
    // The URL we want to download (without "http://")
    let host = "example.com";
    let path = "/";
    
    // Establish a TCP connection to the server
    let mut stream = TcpStream::connect((host, 80))?;

    // Send an HTTP GET request
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response from the server
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    // Convert the response to a string
    let response = String::from_utf8_lossy(&response);

    // Separate headers from the body
    if let Some(body_start) = response.find("\r\n\r\n") {
        let body = &response[(body_start + 4)..];

        // Write the body to a file
        let mut file = File::create("downloaded.html")?;
        file.write_all(body.as_bytes())?;
    } else {
        eprintln!("Failed to find the body in the response");
    }

    Ok(())
}
struct TestFmtEmpty(u32);
impl Debug for TestFmtEmpty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        test_fomratter(f);
        unsafe{printf("addr of self is %p\n\0".as_ptr() as *const i8,self as *const _ as usize);}
        f.write_str("TestFmtEmpty")
    }
}

fn test_fomratter<'a>(fmt: &std::fmt::Formatter<'a>) {
    use std::io::Write;
    
    let raw_width: (usize, usize) = unsafe { std::mem::transmute(fmt.width()) };
    unsafe{printf("raw_width is %p,tag is %p.\n\0".as_ptr() as *const i8,raw_width.0,raw_width.1);
    printf("addr is %p\n\0".as_ptr() as *const i8,fmt as *const _ as usize);}
    match fmt.align() {
        Some(_) => eprintln!("align is Some"),
        None => eprintln!("align is None"),
    }
    match fmt.width() {
        Some(width) => {
            eprintln!("Some, ");
            std::io::stderr().write_all(width.to_string().as_bytes());
        }
        None => eprintln!("None"),
    }
}
fn test_writr_str<'a>(fmt: &mut std::fmt::Formatter<'a>) {
    eprintln!("Testing write_str");
    fmt.write_str("Bob.");
    test_fomratter(&*fmt);
}
extern "C" {
    fn printf(_: *const core::ffi::c_char, _: ...) -> core::ffi::c_int;
}
#[test]
fn test(){
    rustc_clr_interop_managed_call0_::<"System.Console","System.Console",false,"WriteLine",()>();
    rustc_clr_interop_managed_call1_::<"System.Console","System.Console",false,"WriteLine",true,(),i32>(64);
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_call0_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,Ret>()->Ret{
    core::intrinsics::abort();
}
#[allow(dead_code)]
#[inline(never)]
fn rustc_clr_interop_managed_call1_<const ASSEMBLY:&'static str,const CLASS_PATH:&'static str,const IS_VALUETYPE:bool,const METHOD:&'static str,const IS_STATIC:bool,Ret,Arg1>(arg1:Arg1)->Ret{
    core::intrinsics::abort();
}