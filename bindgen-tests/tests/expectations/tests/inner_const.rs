#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo3BOOE"]
    pub static mut Foo_BOO: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZN3Foo8whateverE"]
    pub static mut Foo_whatever: Foo;
}
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::bar"][::std::mem::offset_of!(Foo, bar) - 0usize];
};
