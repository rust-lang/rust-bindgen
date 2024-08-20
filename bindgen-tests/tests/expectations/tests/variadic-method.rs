#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    #[link_name = "\u{1}_Z3fooPKcz"]
    pub fn foo(fmt: *const ::std::os::raw::c_char, ...);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 1usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN3Bar3fooEPKcz"]
    pub fn Bar_foo(this: *mut Bar, fmt: *const ::std::os::raw::c_char, ...);
}
