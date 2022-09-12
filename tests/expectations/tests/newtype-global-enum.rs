#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const Foo_Bar: Foo = Foo(2);
pub const Foo_Baz: Foo = Foo(4);
pub const Foo_Duplicated: Foo = Foo(4);
pub const Foo_Negative: Foo = Foo(-3);
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Foo(pub ::std::os::raw::c_int);
