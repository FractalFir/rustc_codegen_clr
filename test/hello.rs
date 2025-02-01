#![feature(lang_items)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
extern "C" {
    fn puts(string:*const u8);
    fn malloc(size:usize)->*mut ();
    fn free(ptr:*const ());
    fn realloc(ptr:*const (),size:usize)->*const ();
}

struct ASCIStr{
    ptr:*mut u8,
    len:isize,
    cap:isize,
}
impl ASCIStr{
    fn new()->Self{
        Self{ptr:core::ptr::null_mut(),len:0,cap:0}
    }
    fn ptr(&self)->*const u8{
        self.ptr
    }
    fn push(&mut self,ch:u8){
        let new_len = self.len + 1;
        if self.cap >= new_len{
            self.cap = self.cap<<1;
            if self.cap < 4{
                self.cap = 4;
            }
            self.ptr = unsafe{realloc(self.ptr.cast(),self.cap as usize) as *const _ as *mut _};
        }
        unsafe{*self.ptr.offset(self.len) = ch};
        self.len = new_len;
    }
    fn push_char(&mut self,ch:char){
        let val = ch as u32;
        if val > 0x7F{
            self.push(b'?');
        }
        else{
            self.push(val as u8);
        }
    }
}

impl Drop for ASCIStr{
    fn drop(&mut self){
        unsafe{free(self.ptr().cast())};
    }
}
struct Generic<T>{
    arg:T,
}

fn main() {
    unsafe{
        let mut asci_str = ASCIStr::new();
        asci_str.push_char('R');
        asci_str.push_char('u');
        asci_str.push_char('s');
        asci_str.push_char('t');
        asci_str.push_char(' ');
        asci_str.push_char('s');
        asci_str.push_char('a');
        asci_str.push_char('y');
        asci_str.push_char('s');
        asci_str.push_char(':');
        asci_str.push_char('\n');
        asci_str.push_char('"');
        asci_str.push_char('H');
        asci_str.push_char('e');
        asci_str.push_char('l');
        asci_str.push_char('l');
        asci_str.push_char('o');
        asci_str.push_char(',');
        asci_str.push_char(' ');
        asci_str.push_char('.');
        asci_str.push_char('N');
        asci_str.push_char('E');
        asci_str.push_char('T');
        asci_str.push_char('!');
        asci_str.push_char('"');
        asci_str.push_char('\n');
        asci_str.push(0x00);
        
        puts(asci_str.ptr());
        puts(asci_str.ptr());
        puts(asci_str.ptr());
        //free(asci_str.ptr().cast());
    }
    0
}