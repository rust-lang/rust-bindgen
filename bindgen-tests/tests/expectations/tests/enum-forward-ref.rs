#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const Foo_A: Foo = 0;
pub type Foo = ::std::os::raw::c_uint;
pub const Bar_B: Bar = 0;
pub type Bar = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn f() -> Baz;
}
pub const Baz_C: Baz = 0;
pub type Baz = ::std::os::raw::c_uint;
pub type Qux = i32;
