#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub arr: [u32; 3usize],
}
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 12usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::arr"][::std::mem::offset_of!(C, arr) - 0usize];
};
