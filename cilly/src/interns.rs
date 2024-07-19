use std::sync::OnceLock;

union Interns {
    owned: std::mem::ManuallyDrop<Box<str>>,
    interned: &'static str,
}
impl Drop for Interns {
    fn drop(&mut self) {
        unsafe {
            if !get_interned().is_slice_within(self.interned.as_bytes()) {
                std::mem::ManuallyDrop::drop(&mut self.owned)
            }
        }
    }
}
static INTERNED: OnceLock<Interned> = OnceLock::new();
fn get_interned() -> &'static Interned {
    INTERNED.get_or_init(Interned::default)
}
struct Interned {
    /// The start of the backing buffer
    start_ptr: usize,
    /// The end of the backing buffer
    end_ptr: usize,
    min_len: usize,
    max_len: usize,
    raw_strs: &'static [&'static str],
}
impl Interned {
    /// Checks if pointer `ptr` points to a location inside the backing buffer
    pub fn is_ptr_within(&self, ptr: *const u8) -> bool {
        self.start_ptr < ptr as usize && self.end_ptr > ptr as usize
    }
    /// Checks if slice points to a location inside the backing buffer
    pub fn is_slice_within(&self, slice: &[u8]) -> bool {
        self.is_ptr_within(slice.as_ptr())
    }
}
impl Interned {
    pub fn default() -> Self {
        //let strings = ["f0","f1","f2","f3","f4","f5","f6","metadata"]
        todo!()
    }
}
