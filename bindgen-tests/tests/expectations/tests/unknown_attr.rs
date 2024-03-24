#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: ::std::os::raw::c_longlong,
}
const _: () = {
    ["Size of max_align_t"][::std::mem::size_of::<max_align_t>() - 32usize];
    ["Alignment of max_align_t"][::std::mem::align_of::<max_align_t>() - 16usize];
    [
        "Offset of field: max_align_t::__clang_max_align_nonce1",
    ][::std::mem::offset_of!(max_align_t, __clang_max_align_nonce1) - 0usize];
    [
        "Offset of field: max_align_t::__clang_max_align_nonce2",
    ][::std::mem::offset_of!(max_align_t, __clang_max_align_nonce2) - 16usize];
};
