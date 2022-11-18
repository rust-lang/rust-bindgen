#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const Foo_A: Foo = 0;
pub const Foo_B: Foo = 1;
pub type Foo = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FooAlias(pub Foo);
pub mod Bar {
    pub type Type = ::std::os::raw::c_uint;
    pub const C: Type = 0;
    pub const D: Type = 1;
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BarAlias(pub Bar::Type);
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Qux {
    E = 0,
    F = 1,
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct QuxAlias(pub Qux);
pub const Baz_G: Baz = 0;
pub const Baz_H: Baz = 1;
pub type Baz = ::std::os::raw::c_uint;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BazAlias(pub Baz);
impl ::std::ops::Deref for BazAlias {
    type Target = Baz;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for BazAlias {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
