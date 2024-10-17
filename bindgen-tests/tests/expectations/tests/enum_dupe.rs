#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
impl Foo {
    pub const Dupe: Foo = Foo::Bar;
}
pub type Foo_ctype = ::std::os::raw::c_uint;
pub const Foo_Bar: Foo_ctype = 1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = 1,
}
