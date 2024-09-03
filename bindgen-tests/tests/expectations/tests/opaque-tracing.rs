#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    #[link_name = "\u{1}_Z3fooP9Container"]
    pub fn foo(c: *mut Container);
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Container {
    pub _bindgen_opaque_blob: [u32; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Container"][::std::mem::size_of::<Container>() - 8usize];
    ["Alignment of Container"][::std::mem::align_of::<Container>() - 4usize];
};
