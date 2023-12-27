use std::io::Write;
extern "C" {
    fn puts(msg: *const u8);
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
    std::hint::black_box(&string);
    unsafe{puts(string.as_ptr())};
    unsafe{puts("Testing some cool shit\n\0".as_ptr())};
    let mut f = std::fs::File::create("foo.txt").unwrap();

    std::hint::black_box(f);
    std::io::stdout().write_all(b"hello world\n").unwrap();
    let s = format!("Hello {}\0",8);
    unsafe{puts(s.as_ptr())};
   
   
    
    let val = std::hint::black_box(*boxed_int);
    let val = std::hint::black_box(string);
}
use mycorrhiza::*;
// This part is some bolierplate code informing the interop layer which types/metods to use.
// It can be written only once and could be a separate crate.
type MonoBehaviour = mychorriza::Class<"UnityEngine","UnityEngine.MonoBehaviour">;
type Debug = mychorriza::Class<"UnityEngine","UnityEngine.Debug">;
// Convienent wrapper around `Debug.Log(string)`
fn debug_log(message:mychorriza::ManagedString){
    // Calls static method named "Log", with one argument, of type ManagedString(C# String), and returning void
    Deubg::static1::<"Log",mychorriza::ManagedString,()>(message);
}
// Macros turn this high-level class declaration into a specialy marked type, which is then transformed into a .NET class declaration
// by the interop layer.
#[class_decl]
#[inherits_from(MonoBehaviour)]
struct MyBeahviour{
 
    // Other kinds of fields work normaly.
    pub counter:u32,
}
impl MyBehaviour{
    fn Start(self){
        // You could use `"I am being intialized!".into()`` and convert a Rust `str` to C# String, but using C# strings directly is faster
        debug_log(mychorriza::string_literal!("I am being intialized!"));
    }
    fn Update(self){
        // You could use `format!("I am being updated. counter is:{counter}!").into()``
        // and convert a Rust `String` to C# String, but using C# strings directly is faster(for interop)
        debug_log(mychorriza::string_format!("I am being updated. counter is:{counter}!"));
        self.counter += 1;
    }
}
