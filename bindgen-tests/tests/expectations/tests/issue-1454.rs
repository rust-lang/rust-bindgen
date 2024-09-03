#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct extern_type;
#[repr(C)]
#[derive(Debug)]
pub struct local_type {
    pub inner: extern_type,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of local_type"][::std::mem::size_of::<local_type>() - 0usize];
    ["Alignment of local_type"][::std::mem::align_of::<local_type>() - 1usize];
    [
        "Offset of field: local_type::inner",
    ][::std::mem::offset_of!(local_type, inner) - 0usize];
};
