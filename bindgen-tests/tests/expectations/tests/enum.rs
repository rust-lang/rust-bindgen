#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: foo__bindgen_ty_1 = 0;
pub const foo_FOO_B: foo__bindgen_ty_1 = 1;
pub type foo__bindgen_ty_1 = ::std::os::raw::c_uint;
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
pub const Foo_Bar: Foo = 0;
pub const Foo_Qux: Foo = 1;
pub type Foo = ::std::os::raw::c_uint;
pub const Neg_MinusOne: Neg = -1;
pub const Neg_One: Neg = 1;
pub type Neg = ::std::os::raw::c_int;
pub const NoDebug_NoDebug1: NoDebug = 0;
pub const NoDebug_NoDebug2: NoDebug = 1;
/// <div rustbindgen nodebug></div>
pub type NoDebug = ::std::os::raw::c_uint;
pub const Debug_Debug1: Debug = 0;
pub const Debug_Debug2: Debug = 1;
/// <div rustbindgen derive="Debug"></div>
pub type Debug = ::std::os::raw::c_uint;
