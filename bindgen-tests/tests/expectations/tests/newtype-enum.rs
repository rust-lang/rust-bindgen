#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl Foo {
    pub const Bar: Foo = Foo(2);
    pub const Baz: Foo = Foo(4);
    pub const Duplicated: Foo = Foo(4);
    pub const Negative: Foo = Foo(-3);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo(pub ::std::os::raw::c_int);
