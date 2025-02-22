#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(transparent)]
pub struct __bindgen_marker_Opaque<T: ?Sized>(T);
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
unsafe extern "C" {
    #[link_name = "\u{1}_Z8take_foo3FooILi3EE"]
    pub fn take_foo(foo: __bindgen_marker_Opaque<u8>);
}
