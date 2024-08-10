# How to find the source code of a broken test?
1. Download the rust compiler source
`git clone https://github.com/rust-lang/rust.git --depth 1000`
2. Go into the library directory
3. Serach for the name of your test `grep -r "broken_test"`
# How to minimze a broken test?
1. After finding a broken test, extract it into a spearte crate.
2. Build the crate into native Rust using `cargo test` and check that it behaves as expcted. 
3. IMPORTANT! Clean the build cache using `cargo clean`. NEVER skip this step, beacause it can lead to suprising results which may hide the real issue.
4. Build the crate using `rustc_codegen_clr` and observe the incorrect results. 
5. Make a small change by removing \ simplifing some code in your example.
6. Clean the build cache using `cargo clean` again before rebuilding to prevent issues. DO NOT SKIP THS STEP.
7. Repeat steps 2-7 untill the test program can no longer be simplified.
8. Create an issue with your simplified broken test.
# List of broken core test:
## Did not compleate:
```
atomic::atomic_access_bool
atomic::bool_
atomic::int_max
atomic::int_min
atomic::int_nand
atomic::int_xor
atomic::ptr_bitops
atomic::uint_max
atomic::uint_min
atomic::uint_nand
atomic::uint_xor
cell::refcell_ref_coercion
future::test_join
hash::test_writer_hasher
iter::adapters::step_by::test_iterator_step_by_nth_try_fold
iter::range::test_range_advance_by
iter::test_monad_laws_left_identity
manually_drop::smoke
num::flt2dec::random::shortest_f32_exhaustive_equivalence_test
num::flt2dec::random::shortest_f64_hard_random_equivalence_test
num::i128::tests::test_saturating_abs
num::i128::tests::test_saturating_neg
num::int_log::checked_ilog
ptr::ptr_metadata
ptr::test_ptr_metadata_in_const
ptr::test_variadic_fnptr
result::result_try_trait_v2_branch
simd::testing
slice::select_nth_unstable
slice::take_in_bounds_max_range_from
slice::take_in_bounds_max_range_to
slice::take_mut_in_bounds_max_range_from
slice::take_mut_in_bounds_max_range_to
slice::take_mut_oob_max_range_to_inclusive
slice::take_oob_max_range_to_inclusive
```
## Failed
```
num::i8::tests::test_pow
iter::adapters::flat_map::test_flat_map_try_folds
num::bignum::test_add_small_overflow
num::i16::tests::test_checked_next_multiple_of
slice::swap_panics::index_b_equals_len
num::bignum::test_mul_small_overflow
mem::uninit_fill_clone_panic_drop
num::i128::tests::test_lots_of_isqrt
iter::adapters::peekable::test_peek_try_folds
num::bignum::test_mul_digits_overflow_1
num::i8::tests::test_from_str_radix
net::ip_addr::ipv6_properties
num::test_int_from_str_overflow
iter::traits::double_ended::test_rev_try_folds
ascii::test_is_ascii_align_size_thoroughly
num::i16::tests::test_pow
slice::swap_panics::index_a_equals_len
net::ip_addr::ipv6_addr_to_string
iter::adapters::flatten::test_flatten_try_folds
num::bignum::test_get_bit_out_of_range
slice::take_last_nonempty
num::bignum::test_add_overflow_1
iter::adapters::skip::test_skip_try_folds
num::i16::tests::test_from_str
net::socket_addr::ipv6_socket_addr_to_string
iter::adapters::map::test_map_try_folds
cell::refcell_unsized
num::f32::min
iter::adapters::cloned::test_cloned_try_folds
num::bignum::test_from_u64_overflow
iter::adapters::flatten::test_flatten_one_shot
slice::take_first_nonempty
num::i16::tests::test_from_str_radix
num::f32::max
option::as_slice
num::u16::tests::test_rotate
iter::adapters::step_by::test_iterator_step_by_nth_try_rfold
iter::range::test_range_inclusive_folds
iter::adapters::take_while::test_take_while_folds
iter::adapters::take::test_take_try_folds
num::u16::tests::test_leading_trailing_ones
slice::swap_panics::index_b_greater_than_len
iter::adapters::skip_while::test_skip_while_try_fold
slice::take_first_mut_nonempty
num::f64::max
net::socket_addr::socket_v6_to_str
num::i16::tests::test_rotate
slice::swap_panics::index_a_greater_than_len
num::i8::tests::test_checked_next_multiple_of
num::bignum::test_mul_digits_overflow_2
num::f64::min
num::i8::tests::test_from_str
slice::take_last_mut_nonempty
iter::adapters::filter_map::test_filter_map_try_folds
ptr::from_raw_parts
num::u8::tests::test_leading_trailing_ones
num::i16::tests::test_leading_trailing_ones
num::i8::tests::test_leading_trailing_ones
iter::adapters::flatten::test_flatten_one_shot_rev
num::bignum::test_mul_pow5_overflow_2
num::bignum::test_add_overflow_2
num::u128::tests::test_pow
result::result_const
iter::adapters::filter::test_filter_try_folds
num::bignum::test_mul_pow2_overflow_2
num::i128::tests::test_pow
```
# List of broken alloc tests:
## Did not compleate:
```
arc::make_mut_unsized
arc::shared_from_iter_normal
arc::shared_from_iter_trustedlen_no_fuse
arc::shared_from_iter_trustedlen_normal
arc::shared_from_iter_trustedlen_panic
arc::slice
arc::trait_object
arc::uninhabited
autotraits::test_binary_heap
autotraits::test_btree_map
autotraits::test_btree_set
autotraits::test_linked_list
autotraits::test_vec_deque
borrow::test_from_cow_c_str
borrow::test_from_cow_os_str
borrow::test_from_cow_path
borrow::test_from_cow_slice
borrow::test_from_cow_str
heap::alloc_system_overaligned_request
rc::shared_from_iter_normal
rc::shared_from_iter_trustedlen_no_fuse
rc::shared_from_iter_trustedlen_normal
rc::shared_from_iter_trustedlen_panic
rc::slice
rc::trait_object
rc::uninhabited
slice::subslice_patterns
slice::test_split_last
string::test_try_reserve
task::test_local_waker_will_wake_clone
task::test_waker_will_wake_clone
thin_box::align1zst
thin_box::align2zst
thin_box::align64_size_not_pow2
thin_box::align64big
thin_box::align64med
thin_box::align64small
thin_box::align64zst
vec::test_try_reserve
vec_deque::test_try_reserve
```
## Failed:
```
slice::test_split_first_mut
vec::test_index_out_of_bounds
vec::vec_macro_repeating_null_raw_fat_pointer
slice::test_split_first
vec::extract_if_unconsumed_panic
vec::extract_if_consumed_panic
vec_deque::test_try_rfold_moves_iter
vec_deque::test_try_fold_moves_iter
str::const_str_ptr
vec::test_collect_after_iterator_clone
```
