#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: foo__bindgen_ty_1 = foo__bindgen_ty_1::FOO_A;
pub const foo_FOO_B: foo__bindgen_ty_1 = foo__bindgen_ty_1::FOO_B;
#[repr(u32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum foo__bindgen_ty_1 {
    FOO_A = 0,
    FOO_B = 1,
}
const _: () = {
    ["Size of foo"][::std::mem::size_of::<foo>() - 4usize];
    ["Alignment of foo"][::std::mem::align_of::<foo>() - 4usize];
    ["Offset of field: foo::member"][::std::mem::offset_of!(foo, member) - 0usize];
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(u32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Foo {
    Bar = 0,
    Qux = 1,
}
pub mod Neg {
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
#[repr(u32)]
/// <div rustbindgen nodebug></div>
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum NoDebug {
    NoDebug1 = 0,
    NoDebug2 = 1,
}
#[repr(u32)]
/// <div rustbindgen derive="Debug"></div>
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Debug {
    Debug1 = 0,
    Debug2 = 1,
}
