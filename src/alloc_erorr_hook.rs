use std::alloc::Layout;

pub fn custom_alloc_error_hook(layout: Layout) {
    panic!("memory allocation of {} bytes failed", layout.size());
}
