#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const Foo_Bar: Foo = 2;
pub const Foo_Baz: Foo = 4;
pub const Foo_Duplicated: Foo = 4;
pub const Foo_Negative: Foo = -3;
pub type Foo = ::std::os::raw::c_int;
