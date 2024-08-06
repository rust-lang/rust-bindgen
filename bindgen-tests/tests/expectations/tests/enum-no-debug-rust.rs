#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: foo__bindgen_ty_1 = foo__bindgen_ty_1::FOO_A;
pub const foo_FOO_B: foo__bindgen_ty_1 = foo__bindgen_ty_1::FOO_B;
pub type foo__bindgen_ty_1_ctype = ::std::os::raw::c_uint;
pub const foo__bindgen_ty_1_FOO_A: foo__bindgen_ty_1_ctype = 0;
pub const foo__bindgen_ty_1_FOO_B: foo__bindgen_ty_1_ctype = 1;
#[repr(u32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum foo__bindgen_ty_1 {
    FOO_A = 0,
    FOO_B = 1,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
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
pub type Foo_ctype = ::std::os::raw::c_uint;
pub const Foo_Bar: Foo_ctype = 0;
pub const Foo_Qux: Foo_ctype = 1;
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
pub type NoDebug_ctype = ::std::os::raw::c_uint;
pub const NoDebug_NoDebug1: NoDebug_ctype = 0;
pub const NoDebug_NoDebug2: NoDebug_ctype = 1;
#[repr(u32)]
/// <div rustbindgen nodebug></div>
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum NoDebug {
    NoDebug1 = 0,
    NoDebug2 = 1,
}
pub type Debug_ctype = ::std::os::raw::c_uint;
pub const Debug_Debug1: Debug_ctype = 0;
pub const Debug_Debug2: Debug_ctype = 1;
#[repr(u32)]
/// <div rustbindgen derive="Debug"></div>
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Debug {
    Debug1 = 0,
    Debug2 = 1,
}
