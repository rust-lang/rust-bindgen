#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {}
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 0usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 1usize, "Alignment of foo");
};
pub type bar = foo;
