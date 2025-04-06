#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params,
    pointer_is_aligned_to
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
#[inline]
pub  fn is_ascii_simple(mut bytes: &[u8]) -> bool {
    unsafe{printf(c"start:%p len:%lu\n".as_ptr(),&bytes[0] as *const _ as u64,bytes.len() as u64)};
    while let [rest @ .., last] = bytes {
        unsafe{printf(c"%p %lu\n".as_ptr(),last as *const _ as u64,bytes.len() as u64)};
        if !last.is_ascii() {
            break;
        }
        bytes = rest;

    }
    bytes.is_empty()
}
#[inline]
fn is_ascii(s: &[u8]) -> bool {
     /// Returns `true` if any byte in the word `v` is nonascii (>= 128). Snarfed
            /// from `../str/mod.rs`, which does something similar for utf8 validation.
            const fn contains_nonascii(v: usize) -> bool {
                const NONASCII_MASK: usize = usize::from_le_bytes([0x80,0x80,0x80,0x80,0x80,0x80,0x80,0x80]);
                (NONASCII_MASK & v) != 0
            }
            const USIZE_SIZE: usize = size_of::<usize>();
            let len = s.len();
            let align_offset = s.as_ptr().align_offset(USIZE_SIZE);
            // We always read the first word unaligned, which means `align_offset` is
            // 0, we'd read the same value again for the aligned read.
            let offset_to_aligned = if align_offset == 0 { USIZE_SIZE } else { align_offset };
            if len < USIZE_SIZE || len < align_offset || USIZE_SIZE < align_of::<usize>() {
                return is_ascii_simple(s);
            }
            let start = s.as_ptr();
            // SAFETY: We verify `len < USIZE_SIZE` above.
            let first_word = unsafe { (start as *const usize).read_unaligned() };
            if contains_nonascii(first_word) {
                return false;
            }
            // We checked this above, somewhat implicitly. Note that `offset_to_aligned`
            // is either `align_offset` or `USIZE_SIZE`, both of are explicitly checked
            // above.
            test!(offset_to_aligned <= len);
            // SAFETY: word_ptr is the (properly aligned) usize ptr we use to read the
            // middle chunk of the slice.
            let mut word_ptr = unsafe { start.add(offset_to_aligned) as *const usize };
            // `byte_pos` is the byte index of `word_ptr`, used for loop end checks.
            let mut byte_pos = offset_to_aligned;
            // Paranoia check about alignment, since we're about to do a bunch of
            // unaligned loads. In practice this should be impossible barring a bug in
            // `align_offset` though.
            // While this method is allowed to spuriously fail in CTFE, if it doesn't
            // have alignment information it should have given a `usize::MAX` for
            // `align_offset` earlier, sending things through the scalar path instead of
            // this one, so this check should pass if it's reachable.
            test!(word_ptr.is_aligned_to(align_of::<usize>()));
            // Read subsequent words until the last aligned word, excluding the last
            // aligned word by itself to be done in tail check later, to ensure that
            // tail is always one `usize` at most to extra branch `byte_pos == len`.
            while byte_pos < len - USIZE_SIZE {
                // Sanity check that the read is in bounds
                test!(byte_pos + USIZE_SIZE <= len);
                // And that our assumptions about `byte_pos` hold.
                test!(word_ptr.cast::<u8>() == start.wrapping_add(byte_pos));
                // SAFETY: We know `word_ptr` is properly aligned (because of
                // `align_offset`), and we know that we have enough bytes between `word_ptr` and the end
                let word = unsafe { word_ptr.read() };
                if contains_nonascii(word) {
                    return false;
                }
                byte_pos += USIZE_SIZE;
                // SAFETY: We know that `byte_pos <= len - USIZE_SIZE`, which means that
                // after this `add`, `word_ptr` will be at most one-past-the-end.
                word_ptr = unsafe { word_ptr.add(1) };
            }
            // Sanity check to ensure there really is only one `usize` left. This should
            // be guaranteed by our loop condition.
            test!(byte_pos <= len && len - byte_pos <= USIZE_SIZE);
            // SAFETY: This relies on `len >= USIZE_SIZE`, which we check at the start.
            let last_word = unsafe { (start.add(len - USIZE_SIZE) as *const usize).read_unaligned() };
            !contains_nonascii(last_word)
}
// The "obviously-correct" baseline mentioned above.
fn is_ascii_baseline(s: &[u8]) -> bool {
    s.iter().all(|b| b.is_ascii())
}
// Helper to repeat `l` copies of `b0` followed by `l` copies of `b1`.
fn repeat_concat(b0: u8, b1: u8, l: usize) ->Vecy<u8>{
    let mut buff = Vecy::new(0, l* 2);
    for i in 0..l{
        buff.as_slice_mut()[i] = b0;
        buff.as_slice_mut()[i + l] = b1;
    }
   buff
}
// Helper to repeat `l` copies of `b0` followed by `l` copies of `b1`.
fn repeat(b0: u8,l: usize) -> Vecy<u8>{
    let mut buff = Vecy::new(0, l);
    for i in 0..l{
        buff.as_slice_mut()[i] = b0;
    }
    buff
}
fn test_last(bytes:&[u8]){
    if let [rest @ .., last] = (bytes){
        test_eq!(last, &bytes[0]);
        test_eq!(*last, b'a');
    }
}
fn subslice_from_end(s:&[u8])->&[u8]{
    if let [rest @ .., last] = s{
       return rest;
   }
   s
}
fn main(){
    let bytes = b"a";
   test_eq!(*std::hint::black_box(last(b"a")),b'a');
   test_eq!(std::hint::black_box(get_last(b"a")),b'a');
   test_last(std::hint::black_box(bytes));
   
test!(is_ascii_simple(b"a"));
let iter = 0..100;
test_eq!(subslice_from_end(std::hint::black_box(&b"ba"[..])).len(),1);
test_eq!(subslice_from_end(std::hint::black_box(&b"ba"[..]))[0], b'b');
#[cfg(not(debug_assertions))]
test_eq!(subslice_from_end(std::hint::black_box(&b"ba"[..])), &b"b"[..]);
#[cfg(not(debug_assertions))]
test_eq!(&std::hint::black_box(&b"ba"[..])[..1], &b"b"[..]);
for i in iter {
    let cases = [
        repeat(b'a',i),
        /*repeat(b'\0',i),
        repeat(b'\x7f',i),
        repeat(b'\x80',i),
        repeat(b'\xff',i),
        repeat_concat(b'a', 0x80u8, i),
        repeat_concat(0x80u8, b'a', i),*/
    ];
    //test(b"aaaaaaaaa",b"aaaaaaaaa",b"aaaaaaaaa");
   for case in cases {
        let case = case.as_slice();
        for pos in 0..=case.len() {
            // Potentially misaligned head
            let prefix = &case[pos..];
               // Potentially misaligned tail
    let suffix = &case[..case.len() - pos];
        // Both head and tail are potentially misaligned
    let mid = &case[(pos / 2)..(case.len() - (pos / 2))];
            test(prefix,suffix,mid);
        }
    }
}
}
#[no_mangle]
fn last(s:&[u8])->&u8{
    let [.., last] = s else {
        unsafe{std::hint::unreachable_unchecked()};
    };
    last
}
#[no_mangle]
fn get_last(s:&[u8])->u8{
    let [.., last] = s else {
        unsafe{std::hint::unreachable_unchecked()};
    };
    *last
}
fn test(prefix:&[u8],suffix:&[u8], mid:&[u8]){
    test_eq!(is_ascii_baseline(prefix), is_ascii(prefix));
    test_eq!(is_ascii_baseline(suffix), is_ascii(suffix));
    test_eq!(is_ascii_baseline(mid), is_ascii(mid));
}