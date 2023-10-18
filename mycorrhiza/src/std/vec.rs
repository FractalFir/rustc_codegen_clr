use core::ptr;
type Marshal = crate::intrinsics::RustcCLRInteropManagedClass<
    "System.Runtime.InteropServices",
    "System.Runtime.InteropServices.Marshal",
>;
pub struct Vec<T> {
    buf: *mut T,
    cap: usize,
    len: usize,
}
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        for index in 0..self.cap {
            unsafe { core::ptr::drop_in_place(self.buf.add(index)) };
        }
    }
}
impl<T> Vec<T> {
    /// Resizes the internal buffer to have the capacity for exactly `new_cap` elements. WARNING: This does not call drop nor initalizes the memory.
    unsafe fn resize_buff_to(&mut self, new_cap: usize) {
        let new_cap_in_bytes = new_cap * core::mem::size_of::<T>();
        self.buf = Marshal::static2::<"ReAllocHGlobal", usize, usize, usize>(
            self.buf as usize,
            new_cap_in_bytes,
        ) as *mut T;
        self.cap = new_cap;
    }
    /// Expands the buf to have capacity for at least len*2 items.
    fn reserve_for_push(&mut self) {
        if self.cap < 4 {
            unsafe { self.resize_buff_to(4) };
        } else {
            unsafe { self.resize_buff_to(self.cap * 2) };
        }
    }
    /// Pushes a new element into a vector.
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.reserve_for_push();
        }
        unsafe {
            let end = self.buf.add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    /// Pops an element from a vector
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(unsafe { core::ptr::read(self.buf.add(self.len)) })
        }
    }
}
