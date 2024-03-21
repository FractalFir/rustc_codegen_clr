#![feature(allocator_api,try_reserve_kind,sized_type_properties,ptr_internals,core_intrinsics,alloc_layout_extra,unchecked_math)]
use std::mem;
use std::collections::TryReserveError;
use std::mem::SizedTypeProperties;
use std::alloc::AllocError;
use std::collections::TryReserveErrorKind::CapacityOverflow;
use std::ptr::Unique;
use std::ptr::NonNull;
use std::hint::black_box;
struct LayoutError;
#[derive(Copy,Clone)]
pub struct Alignment(AlignmentEnum);
#[repr(usize)]
#[derive(Copy,Clone)]
enum AlignmentEnum {
    _Align1Shl0 = 1 << 0,
    _Align1Shl1 = 1 << 1,
    _Align1Shl2 = 1 << 2,
    _Align1Shl3 = 1 << 3,
    _Align1Shl4 = 1 << 4,
    _Align1Shl5 = 1 << 5,
    _Align1Shl6 = 1 << 6,
    _Align1Shl7 = 1 << 7,
    _Align1Shl8 = 1 << 8,
    _Align1Shl9 = 1 << 9,
    _Align1Shl10 = 1 << 10,
    _Align1Shl11 = 1 << 11,
    _Align1Shl12 = 1 << 12,
    _Align1Shl13 = 1 << 13,
    _Align1Shl14 = 1 << 14,
    _Align1Shl15 = 1 << 15,
    _Align1Shl16 = 1 << 16,
    _Align1Shl17 = 1 << 17,
    _Align1Shl18 = 1 << 18,
    _Align1Shl19 = 1 << 19,
    _Align1Shl20 = 1 << 20,
    _Align1Shl21 = 1 << 21,
    _Align1Shl22 = 1 << 22,
    _Align1Shl23 = 1 << 23,
    _Align1Shl24 = 1 << 24,
    _Align1Shl25 = 1 << 25,
    _Align1Shl26 = 1 << 26,
    _Align1Shl27 = 1 << 27,
    _Align1Shl28 = 1 << 28,
    _Align1Shl29 = 1 << 29,
    _Align1Shl30 = 1 << 30,
    _Align1Shl31 = 1 << 31,
}
const fn size_align<T>() -> (usize, usize) {
    (core::mem::size_of::<T>(), core::mem::align_of::<T>())
}
pub const fn invalid_mut<T>(addr: usize) -> *mut T {
    // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
    // We use transmute rather than a cast so tools like Miri can tell that this
    // is *not* the same as from_exposed_addr.
    // SAFETY: every valid integer is also a valid pointer (as long as you don't dereference that
    // pointer).
    unsafe { mem::transmute(addr) }
}
pub struct Layout {
    // size of the requested block of memory, measured in bytes.
    size: usize,

    // alignment of the requested block of memory, measured in bytes.
    // we ensure that this is always a power-of-two, because API's
    // like `posix_memalign` require it and it is a reasonable
    // constraint to impose on Layout constructors.
    //
    // (However, we do not analogously require `align >= sizeof(void*)`,
    //  even though that is *also* a requirement of `posix_memalign`.)
    align: Alignment,
}

impl Alignment{
    #[inline]
    pub const fn new(align: usize) -> Option<Self> {
        if align.is_power_of_two() {
            // SAFETY: Just checked it only has one bit set
            Some(unsafe { Self::new_unchecked(align) })
        } else {
            None
        }
    }
    pub const fn as_usize(&self)->usize{
        unsafe { mem::transmute::<Alignment,usize>(*self) }
    }
    pub const unsafe fn new_unchecked(align: usize) -> Self {

        // SAFETY: By precondition, this must be a power of two, and
        // our variants encompass all possible powers of two.
        unsafe { mem::transmute::<usize, Alignment>(align) }
    }
    pub const fn of<T>() -> Self {
        // SAFETY: rustc ensures that type alignment is always a power of two.
        unsafe { Alignment::new_unchecked(mem::align_of::<T>()) }
    }
}
impl Layout{
    #[inline(always)]
    const fn max_size_for_align(align: Alignment) -> usize {
        // (power-of-two implies align != 0.)

        // Rounded up size is:
        //   size_rounded_up = (size + align - 1) & !(align - 1);
        //
        // We know from above that align != 0. If adding (align - 1)
        // does not overflow, then rounding up will be fine.
        //
        // Conversely, &-masking with !(align - 1) will subtract off
        // only low-order-bits. Thus if overflow occurs with the sum,
        // the &-mask cannot subtract enough to undo that overflow.
        //
        // Above implies that checking for summation overflow is both
        // necessary and sufficient.
        isize::MAX as usize - (align.as_usize() - 1)
    }

    #[inline]
    pub const fn array<T>(n: usize) -> Result<Self, LayoutError> {
        // Reduce the amount of code we need to monomorphize per `T`.
        return inner(mem::size_of::<T>(), Alignment::of::<T>(), n);

        #[inline]
        const fn inner(
            element_size: usize,
            align: Alignment,
            n: usize,
        ) -> Result<Layout, LayoutError> {
            // We need to check two things about the size:
            //  - That the total size won't overflow a `usize`, and
            //  - That the total size still fits in an `isize`.
            // By using division we can check them both with a single threshold.
            // That'd usually be a bad idea, but thankfully here the element size
            // and alignment are constants, so the compiler will fold all of it.
            if element_size != 0 && n > Layout::max_size_for_align(align) / element_size {
                return Err(LayoutError);
            }

            // SAFETY: We just checked that we won't overflow `usize` when we multiply.
            // This is a useless hint inside this function, but after inlining this helps
            // deduplicate checks for whether the overall capacity is zero (e.g., in RawVec's
            // allocation path) before/after this multiplication.
            let array_size = unsafe { element_size.unchecked_mul(n) };

            // SAFETY: We just checked above that the `array_size` will not
            // exceed `isize::MAX` even when rounded up to the alignment.
            // And `Alignment` guarantees it's a power of two.
            unsafe { Ok(Layout::from_size_align_unchecked(array_size, align.as_usize())) }
        }
    }
    pub fn test_layout(){
        unsafe{
        let mut res = Self::from_size_align_unchecked(0,1024);
        res.size = 64;
        //test_eq!(res.size,64);
        res.align =  Alignment::new_unchecked(1024);
        //test_eq!(res.align(),1024);
        //test_eq!(res.size,64);
        }
    }
    #[must_use]
    #[inline]
    pub fn dangling(&self) -> NonNull<u8> {
        // SAFETY: align is guaranteed to be non-zero
        //f64::putnl(77.6655443322);
        unsafe { NonNull::new_unchecked(invalid_mut::<u8>(self.align())) }
    }
    #[must_use]
    #[inline]
    pub fn new<T>() -> Self {
        let (size, align) = size_align::<T>();
        //f64::putnl(642.544);
        //u64::putnl(size as u64);
        // SAFETY: if the type is instantiated, rustc already ensures that its
        // layout is valid. Use the unchecked constructor to avoid inserting a
        // panicking codepath that needs to be optimized out.
        //test_ne!(size,0);
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }
    #[must_use]
    #[inline]
    pub const unsafe fn from_size_align_unchecked(size: usize, align: usize) -> Self {
        // SAFETY: the caller is required to uphold the preconditions.
        unsafe { Layout { size, align: Alignment::new_unchecked(align) } }
    }
    /* 
    #[must_use]
    #[inline]
    pub fn for_value<T>(t: &T) -> Self {
        let (size, align) = size_align::<T>();
        // SAFETY: see rationale in `new` for why this is using the unsafe variant
        f64::putnl(552.544);
        u64::putnl(size as u64);
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }*/
    #[inline]
    pub fn align(&self) -> usize {
        self.align.as_usize()
    }
    pub fn size(&self) -> usize {
        //f64::putnl(111.544);
        //u64::putnl(self.size as u64);
        self.size
    }
}
#[allow(dead_code)]
extern "C"{
    fn puts(msg:*const u8);
    fn malloc(size:usize)->*mut core::ffi::c_void;
    fn free(ptr:*mut core::ffi::c_void);
    fn realloc(ptr:*mut core::ffi::c_void,size:usize)->*mut core::ffi::c_void;
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    fn printf(fmt: *const core::ffi::c_char, ...) -> core::ffi::c_int;
} 
enum AllocInit {
    /// The contents of the new memory are uninitialized.
    Uninitialized,
    #[cfg(not(no_global_oom_handling))]
    /// The new memory is guaranteed to be zeroed.
    Zeroed,
}

pub(crate) struct RawVec<T> {
    ptr: Unique<T>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: isize,
    alloc: Allocator,
}
impl<T> RawVec<T> {

    pub const fn new_in(alloc: Allocator) -> Self {
        // `cap: 0` means "unallocated". zero-sized types are ignored.
        Self { ptr: Unique::dangling(), cap: 0, alloc }
    }
    fn try_allocate_in_(
        capacity: usize,
    
        alloc: Allocator,
    ) -> Result<Self, TryReserveError> {
        // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.

        // We avoid `unwrap_or_else` here because it bloats the amount of
            // LLVM IR generated.
            let layout = match Layout::array::<T>(capacity) {
                Ok(layout) => layout,
                Err(_) => return Err(CapacityOverflow.into()),
            };

            if let Err(err) = alloc_guard(layout.size()) {
                return Err(err);
            }

            
            // Allocators currently return a `NonNull<[u8]>` whose length
            // matches the size requested. If that ever changes, the capacity
            // here should change to `ptr.len() / mem::size_of::<T>()`.
            black_box(6);
            Ok(Self::new_in(alloc))
    }
    fn try_allocate_in(
        capacity: usize,
        init: AllocInit,
        alloc: Allocator,
    ) -> Result<Self, TryReserveError> {
        // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.

        if T::IS_ZST || capacity == 0 {
            Ok(Self::new_in(alloc))
        } else {
            // We avoid `unwrap_or_else` here because it bloats the amount of
            // LLVM IR generated.
            let layout = match Layout::array::<T>(capacity) {
                Ok(layout) => layout,
                Err(_) => return Err(CapacityOverflow.into()),
            };

            if let Err(err) = alloc_guard(layout.size()) {
                return Err(err);
            }

            let result = match init {
                AllocInit::Uninitialized => alloc.allocate(layout),
                #[cfg(not(no_global_oom_handling))]
                AllocInit::Zeroed => alloc.allocate_zeroed(layout),
            };
            let ptr = match result {
                Ok(ptr) => ptr,
                Err(_) => return Err(panic!()),
            };

            // Allocators currently return a `NonNull<[u8]>` whose length
            // matches the size requested. If that ever changes, the capacity
            // here should change to `ptr.len() / mem::size_of::<T>()`.
            Ok(Self { ptr: Unique::from(ptr.cast()), cap: unsafe { capacity as isize }, alloc })
        }
    }
    pub fn try_with_capacity_in(capacity: usize, alloc: Allocator) -> Result<Self, TryReserveError> {
        Self::try_allocate_in_(capacity, alloc)
    }
}
struct Allocator;
impl Allocator{

        #[inline]
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            //test_ne!(layout.size(),0);
            self.alloc_impl(layout, false)
        }
        #[inline]
        fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            //test_ne!(layout.size(),0);
            let mut res = self.alloc_impl(layout, false)?;
            std::hint::black_box(&mut res);
            Ok(res)
        }
        #[inline]
        fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
            //test_ne!(layout.size(),0);
            match layout.size() {
                0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
                // SAFETY: `layout` is non-zero in size,
                size => unsafe {
                    let raw_ptr = if zeroed {
                        //Self::alloc_zeroed(self, layout)
                        core::intrinsics::abort();
                    } else {
                        Self::alloc(self, layout)
                    };
                    let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;

                    Ok(NonNull::slice_from_raw_parts(ptr, size))
                },
            }
        }
        #[inline]
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // jemalloc provides alignment less than MIN_ALIGN for small allocations.
            // So only rely on MIN_ALIGN if size >= align.
            // Also see <https://github.com/rust-lang/rust/issues/45955> and
            // <https://github.com/rust-lang/rust/issues/62251#issuecomment-507580914>.
            let ptr = if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
                let ptr = malloc(layout.size()) as *mut u8;
                //printf("Malloc retunred ptr: %p\n\0".as_ptr() as *const _,ptr);
                ptr
            } else {
                Self::aligned_malloc(&layout)
            };
 
            ptr
        }
        #[inline]
        unsafe fn aligned_malloc(layout: &Layout) -> *mut u8 {
            // C11 aligned_alloc requires that the size be a multiple of the alignment.
            // Layout already checks that the size rounded up doesn't overflow isize::MAX.
            let align = layout.align();
            let size = layout.size().next_multiple_of(align);
            __rust_alloc(align, size) as *mut u8
        }

}
const MIN_ALIGN:usize = 8;
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(CapacityOverflow.into())
    } else {
        Ok(())
    }
}

fn main(){
    /*let init =  AllocInit::Uninitialized;
    let mut is_zeroed = match init{
        AllocInit::Uninitialized => false,
        AllocInit::Zeroed => true,
    };
    black_box(is_zeroed);
    let layout = match Layout::array::<u64>(4) {
        Ok(layout) => layout,
        Err(_) => panic!(),
    };
    black_box(layout);*/
       // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.

        // We avoid `unwrap_or_else` here because it bloats the amount of
            // LLVM IR generated.
            let layout = match Layout::array::<i32>(4) {
                Ok(layout) => layout,
                Err(_) =>  panic!(),
            };
            black_box(layout.size());
            if let Err(err) = alloc_guard(layout.size()) {
                panic!();
            }

            
            // Allocators currently return a `NonNull<[u8]>` whose length
            // matches the size requested. If that ever changes, the capacity
            // here should change to `ptr.len() / mem::size_of::<T>()`.
            black_box(6);
            let v = RawVec::<u8>::new_in(Allocator);
}
