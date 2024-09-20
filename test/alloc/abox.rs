#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    fundamental,
    ptr_internals,
    sized_type_properties,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_unsafe,
    unused_imports,
    private_interfaces,
    unused_mut
)]
#![no_std]
include!("../common.rs");
use core::mem;
use core::mem::SizedTypeProperties;
use core::ptr::{self, NonNull, Unique};
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
}
impl Layout {
    pub fn test_layout() {
        unsafe {
            let mut res = Self::from_size_align_unchecked(0, 1024);
            res.size = 64;
            test_eq!(res.size, 64);
            res.align = Alignment::new_unchecked(1024);
            test_eq!(res.align(), 1024);
            test_eq!(res.size, 64);
        }
    }
    #[must_use]
    #[inline]
    pub fn dangling(&self) -> NonNull<u8> {
        // SAFETY: align is guaranteed to be non-zero

        unsafe { NonNull::new_unchecked(invalid_mut::<u8>(self.align())) }
    }
    #[must_use]
    #[inline]
    pub fn new<T>() -> Self {
        let (size, align) = size_align::<T>();

        // SAFETY: if the type is instantiated, rustc already ensures that its
        // layout is valid. Use the unchecked constructor to avoid inserting a
        // panicking codepath that needs to be optimized out.
        //test_ne!(size,0);
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }
    #[must_use]
    #[inline]
    pub unsafe fn from_size_align_unchecked(size: usize, align: usize) -> Self {
        // SAFETY: the caller is required to uphold the preconditions.
        unsafe {
            Layout {
                size,
                align: Alignment::new_unchecked(align),
            }
        }
    }
    fn from_size_align(size: usize, align: usize) -> Result<Self, LayoutError> {
        if !align.is_power_of_two() {
            return Err(LayoutError);
        }

        // SAFETY: just checked that align is a power of two.
        Layout::from_size_alignment(size, unsafe { Alignment::new_unchecked(align) })
    }
    /// Internal helper constructor to skip revalidating alignment validity.
    #[inline]
    fn from_size_alignment(size: usize, align: Alignment) -> Result<Self, LayoutError> {
        if size > Self::max_size_for_align(align) {
            return Err(LayoutError);
        }

        // SAFETY: Layout::size invariants checked above.
        Ok(Layout { size, align })
    }
    /*
    #[must_use]
    #[inline]
    pub fn for_value<T>(t: &T) -> Self {
        let (size, align) = size_align::<T>();
        // SAFETY: see rationale in `new` for why this is using the unsafe variant

        unsafe { Layout::from_size_align_unchecked(size, align) }
    }*/
    #[inline]
    pub fn align(&self) -> usize {
        self.align.as_usize()
    }
    pub fn size(&self) -> usize {
        self.size
    }
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
}
pub struct LayoutError;
#[derive(Debug)]
pub struct ScalarSizeMismatch {
    pub target_size: u64,
    pub data_size: u64,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AllocError;
#[lang = "owned_box"]
#[fundamental]
pub struct Box<T>(Unique<T>, Alloc);
impl<T> Box<T> {
    #[must_use]
    #[inline]
    pub fn new_in(x: T, alloc: Alloc) -> Self {
        let mut boxed = Self::new_uninit_in(alloc);
        test_ne!(boxed.as_ptr(), 0_usize as *const T);

        unsafe {
            boxed.as_mut_ptr().write(x);
        }
        unsafe { core::mem::transmute(boxed) }
    }
    pub fn new(x: T) -> Self {
        Self::new_in(x, Alloc)
    }
    #[inline]
    pub const unsafe fn from_raw_in(raw: *mut T, alloc: Alloc) -> Self {
        Box(unsafe { Unique::new_unchecked(raw) }, alloc)
    }
    pub fn try_new_uninit_in(alloc: Alloc) -> Result<Box<mem::MaybeUninit<T>>, AllocError> {
        let ptr = if T::IS_ZST {
            NonNull::dangling()
        } else {
            let layout = Layout::new::<mem::MaybeUninit<T>>();
            //test_ne!(layout.size(),0);
            unsafe { alloc.allocate(layout)?.cast() }
        };

        unsafe { Ok(Box::from_raw_in(ptr.as_ptr(), alloc)) }
    }
    // #[unstable(feature = "new_uninit", issue = "63291")]
    pub fn new_uninit_in(alloc: Alloc) -> Box<mem::MaybeUninit<T>>
where {
        let layout = Layout::new::<mem::MaybeUninit<T>>();
        // NOTE: Prefer match over unwrap_or_else since closure sometimes not inlineable.
        // That would make code size bigger.
        match Box::try_new_uninit_in(alloc) {
            Ok(m) => m,
            Err(_) => handle_alloc_error(layout),
        }
    }
}
pub fn handle_alloc_error(layout: Layout) -> ! {
    core::intrinsics::abort()
}
struct Alloc;
impl Alloc {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        test_ne!(layout.size(), 0);
        self.alloc_impl(layout, false)
    }

    #[inline]
    fn alloc_impl(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
        test_ne!(layout.size(), 0);
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
            printf("Malloc retunred ptr: %p\n\0".as_ptr() as *const _, ptr);
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
fn main() {
    unsafe {
        Layout::test_layout();
        let layout = black_box(Layout::new::<i32>());
        test_eq!(layout.size(), 4_usize);
        let layout = black_box(Layout::from_size_align_unchecked(4, black_box(64)));
        test_eq!(layout.size(), 4_usize);
        test_eq!(8, core::mem::size_of::<AlignmentEnum>());
        test_eq!(8, core::mem::size_of::<Alignment>());
        test_eq!(16, core::mem::size_of::<Layout>());
        let mut layout = Layout::new::<u8>();
        test_eq!(layout.size(), 1);
        let boxed = Box::new(64_u8);
        //main2();
    }
    layout_round_up_to_align_edge_cases();
}
struct UnsizedStruct<T: ?Sized> {
    unsized_field: T,
}
trait TestTrait {}
impl<T> TestTrait for [T] {}
impl<T, const N: usize> TestTrait for [T; N] {}
fn layout_round_up_to_align_edge_cases() {
    const MAX_SIZE: usize = isize::MAX as usize;

    for shift in 0..usize::BITS {
        let align = 1_usize << shift;
        let edge = (MAX_SIZE + 1) - align;
        let low = edge.saturating_sub(10);
        let high = edge.saturating_add(10);

        unsafe {
            printf(
                c"low:%p edge:%p high:%p align:%p\n".as_ptr(),
                low,
                edge,
                high,
                align,
            )
        };
        test!(Layout::from_size_align(low, align).is_ok());
        test!(Layout::from_size_align(high, align).is_err());
        for size in low..=high {
            unsafe { printf(c"size:%d align:%d\n".as_ptr(), size, align) };
            test_eq!(
                Layout::from_size_align(size, align).is_ok(),
                size.next_multiple_of(align) <= MAX_SIZE
            );
        }
    }
}
