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
const _: () = {
    assert!(::std::mem::size_of::<Container>() == 8usize, "Size of Container");
    assert!(::std::mem::align_of::<Container>() == 4usize, "Alignment of Container");
};
