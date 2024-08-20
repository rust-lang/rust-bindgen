#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1::Type,
}
pub mod foo__bindgen_ty_1 {
    pub type Type = ::std::os::raw::c_uint;
    pub const FOO_A: Type = 0;
    pub const FOO_B: Type = 1;
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
pub mod Foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const Bar: Type = 0;
    pub const Qux: Type = 1;
}
pub mod Neg {
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
pub mod NoDebug {
    /// <div rustbindgen nodebug></div>
    pub type Type = ::std::os::raw::c_uint;
    pub const NoDebug1: Type = 0;
    pub const NoDebug2: Type = 1;
}
pub mod Debug {
    /// <div rustbindgen derive="Debug"></div>
    pub type Type = ::std::os::raw::c_uint;
    pub const Debug1: Type = 0;
    pub const Debug2: Type = 1;
}
