#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const true_: u32 = 1;
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo(pub u64);
pub const Foo_A: Foo = Foo(1);
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar(pub ::std::os::raw::c_char);
pub const Bar_A: Bar = Bar(97);
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz(pub f32);
pub const Baz_A: Baz = Baz(3.25);
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bang(pub bool);
pub const Bang_A: Bang = Bang(true);
pub type Boom = u64;
pub const Boom_A: Boom = 2;
