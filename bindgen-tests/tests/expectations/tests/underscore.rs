#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const __: ::std::os::raw::c_int = 10;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ptr_t {
    pub __: [::std::os::raw::c_uchar; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ptr_t"][::std::mem::size_of::<ptr_t>() - 8usize];
    ["Alignment of ptr_t"][::std::mem::align_of::<ptr_t>() - 1usize];
    ["Offset of field: ptr_t::__"][::std::mem::offset_of!(ptr_t, __) - 0usize];
};
