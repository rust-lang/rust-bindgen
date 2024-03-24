#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub foo: usize,
}
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 8usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 8usize];
    ["Offset of field: A::foo"][::std::mem::offset_of!(A, foo) - 0usize];
};
