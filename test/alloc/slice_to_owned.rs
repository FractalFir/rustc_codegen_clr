#![feature(
    allocator_api,
    try_reserve_kind,
    sized_type_properties,
    ptr_internals,
    core_intrinsics,
    alloc_layout_extra
)]
#![allow(internal_features,unreachable_code,unused_unsafe,private_interfaces,dead_code)]
use mem::MaybeUninit;
use slice::SliceIndex;
use std::alloc::AllocError;
use std::borrow::Borrow;
use std::collections::TryReserveError;
use std::collections::TryReserveErrorKind::CapacityOverflow;
use std::hint::black_box;
use std::mem;
use std::mem::SizedTypeProperties;
use std::ops;
use std::ops::Index;
use std::ptr::NonNull;
use std::ptr::Unique;
use std::slice;

struct LayoutError;
#[derive(Copy, Clone)]
pub struct Alignment(AlignmentEnum);
#[repr(usize)]
#[derive(Copy, Clone)]
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

impl Alignment {
    #[inline]
    pub const fn new(align: usize) -> Option<Self> {
        if align.is_power_of_two() {
            // SAFETY: Just checked it only has one bit set
            Some(unsafe { Self::new_unchecked(align) })
        } else {
            None
        }
    }
    pub const fn as_usize(&self) -> usize {
        unsafe { mem::transmute::<Alignment, usize>(*self) }
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
impl Layout {
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
            unsafe {
                Ok(Layout::from_size_align_unchecked(
                    array_size,
                    align.as_usize(),
                ))
            }
        }
    }
    pub fn test_layout() {
        unsafe {
            let mut res = Self::from_size_align_unchecked(0, 1024);
            res.size = 64;
            //test_eq!(res.size,64);
            res.align = Alignment::new_unchecked(1024);
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
        unsafe {
            Layout {
                size,
                align: Alignment::new_unchecked(align),
            }
        }
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
extern "C" {
    fn puts(msg: *const u8);
    fn malloc(size: usize) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn realloc(ptr: *mut core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
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
    len: usize,
}

impl<T> ops::Deref for RawVec<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
}
impl<T> RawVec<T> {
    pub fn capacity(&self) -> usize {
        self.cap as usize
    }
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }
    pub const fn new_in(alloc: Allocator) -> Self {
        // `cap: 0` means "unallocated". zero-sized types are ignored.
        Self {
            ptr: Unique::dangling(),
            cap: 0,
            len: 0,
            alloc,
        }
    }
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        unsafe {
            slice::from_raw_parts_mut(
                self.as_mut_ptr().add(self.len) as *mut MaybeUninit<T>,
                self.capacity() - self.len,
            )
        }
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
            Ok(Self {
                ptr: Unique::from(ptr.cast()),
                cap: unsafe { capacity as isize },
                alloc,
                len: 0,
            })
        }
    }
    pub fn try_with_capacity_in(
        capacity: usize,
        alloc: Allocator,
    ) -> Result<Self, TryReserveError> {
        Self::try_allocate_in(capacity, AllocInit::Uninitialized, alloc)
    }
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }
}
impl<T> Borrow<[T]> for RawVec<T> {
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}
impl<T, I: SliceIndex<[T]>> Index<I> for RawVec<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

struct Allocator;
impl Allocator {
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
const MIN_ALIGN: usize = 8;
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(CapacityOverflow.into())
    } else {
        Ok(())
    }
}

pub const fn as_bytes(stri: &str) -> &[u8] {
    // SAFETY: const sound because we transmute two types with the same layout
    unsafe { mem::transmute(stri) }
}

fn to_vec(s: &[u8], alloc: Allocator) -> RawVec<u8> {
    struct DropGuard<'a> {
        vec: &'a mut RawVec<u8>,
        num_init: usize,
    }
    impl<'a> Drop for DropGuard<'a> {
        #[inline]
        fn drop(&mut self) {
            // SAFETY:
            // items were marked initialized in the loop below
            unsafe {
                self.vec.set_len(self.num_init);
            }
        }
    }
    let mut vec = RawVec::try_with_capacity_in(s.len(), alloc).unwrap();
    let mut guard = DropGuard {
        vec: &mut vec,
        num_init: 0,
    };
    let slots = guard.vec.spare_capacity_mut();
    // .take(slots.len()) is necessary for LLVM to remove bounds checks
    // and has better codegen than zip.
    let count = s.iter().count();
    unsafe {
        printf(
            "Prepraing to coppy. Spare cap %d. Data length %d. Iter count %d\n\0".as_ptr() as *const i8,
            slots.len() as u32,
            s.len() as u32,
            count as u32,
        )
    };
    let mut idx = 0;
    let mut iter = s.iter();
    if !iter.next().is_some(){
        unsafe { printf("Iter should have returned Some but returned None :(.\n\0".as_ptr() as *const i8) };
        core::intrinsics::abort();
    }
    for b in s.iter(){
        unsafe { printf("Copying at %d:%d\n\0".as_ptr() as *const i8,idx as u32,*b as u32) };
        guard.num_init = idx;
      
        //let addr = &mut slots[idx] as *mut _ as *mut u8;
        slots[idx].write(*b);
        //unsafe{*addr = *b};
        
        idx += 1;
    }
    unsafe { printf("Coppy done.\n\0".as_ptr() as *const i8) };
    core::mem::forget(guard);
    // SAFETY:
    // the vec was allocated and initialized above to at least this length.
    unsafe {
        vec.set_len(s.len());
    }
    vec
}
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}
impl<I> Enumerate<I> {
    pub fn new(iter: I) -> Enumerate<I> {
        Enumerate { iter, count: 0 }
    }
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator,
{
    type Item = (usize, <I as Iterator>::Item);

    /// # Overflow Behavior
    ///
    /// The method does no guarding against overflows, so enumerating more than
    /// `usize::MAX` elements either produces the wrong result or panics. If
    /// debug assertions are enabled, a panic is guaranteed.
    ///
    /// # Panics
    ///
    /// Might panic if the index of the element overflows a `usize`.


    fn next(&mut self) -> Option<(usize, <I as Iterator>::Item)> {
        //unsafe{printf("Called next!\n\0".as_ptr() as *const i8)};
        let a = self.iter.next();
        //unsafe{printf("inner itrer `is_some` = %d!\n\0".as_ptr() as *const i8,a.is_some() as u8 as u32)};
        let a = a?;
        //unsafe{printf("is indeed some!\n\0".as_ptr() as *const i8)};
        let i = self.count;
        self.count += 1;

        let res = Some((i, a));
        //unsafe{printf("res `is_some` = %d!\n\0".as_ptr() as *const i8,res.is_some() as u8 as u32)};
        res
    }
}
fn enumerate_<T>(s: T) -> Enumerate<T>
where
    T: Sized,
{
    Enumerate::new(s)
}
fn slice_iter_test(s: &[u8]) {
    for b in s.iter() {
        unsafe { printf("Iter byte %d\n\0".as_ptr() as *const i8, *b as u32) };
    }
}
fn slice_iter_enumerate_test1(s: &[u8]) {
    for (i, b) in enumerate_(s.iter()) {
        unsafe {
            printf(
                "IterEnum1 byte %d at index %d\n\0".as_ptr() as *const i8,
                *b as u32,
                i as u32,
            )
        };
    }
}
fn slice_iter_enumerate_test2(s: &[u8]) {
    
    for (i, b) in s.iter().enumerate() {
        unsafe {
            printf(
                "Iter Enum2 byte %d at index %d\n\0".as_ptr() as *const i8,
                *b as u32,
                i as u32,
            )
        };
    }
}
fn enm_tuple_test(){
    let v = 0;
    let itr:Option<<std::iter::Enumerate<std::slice::Iter<'_, u8>> as Iterator>::Item> = Some((0_usize,&v));
    if !black_box(itr).is_some(){
        unsafe{printf("WTF? Some is... not some?\n\0".as_ptr() as *const i8)};
        core::intrinsics::abort();
    }
    else{
        unsafe{printf("Ok. Some is some.\n\0".as_ptr() as *const i8)};
    }
}
fn uninit_test(){
    unsafe{
   let mut uninit = black_box(MaybeUninit::<u8>::uninit());
   uninit.write(171);
   if uninit.assume_init()  != 171{
   // unsafe{printf("WTF? MaybeUninit is broken.\n\0".as_ptr() as *const i8)};
        core::intrinsics::abort();
   }
    else{
    //unsafe{printf("Direct MaybeUninit is OK.\n\0".as_ptr() as *const i8)};
   }
   //let mut uninit_ref = &mut uninit;
}
}
fn main() {
  /* 
    enm_tuple_test();


    let v = RawVec::<u8>::new_in(Allocator);
    let original = "Hello.\n\0";
    let mut owned = to_vec(original.as_bytes(), Allocator);


    slice_iter_test(original.as_bytes());

    slice_iter_enumerate_test1(original.as_bytes());

    slice_iter_enumerate_test2(original.as_bytes());
    */
    uninit_test();
    /* 
    unsafe { printf(owned.as_mut_ptr() as *const i8) };
    unsafe { printf("\n\0".as_ptr() as *const i8) };
    if (original.len() != owned.len()) {
        core::intrinsics::abort();
    }
    if (original.as_bytes()[0] != owned[0]) {
        core::intrinsics::abort();
    }*/
}
