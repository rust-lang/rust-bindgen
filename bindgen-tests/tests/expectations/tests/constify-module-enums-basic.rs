#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const THIS: Type = 0;
    pub const SHOULD_BE: Type = 1;
    pub const A_CONSTANT: Type = 2;
}
pub use self::foo::Type as foo_alias1;
pub use self::foo_alias1 as foo_alias2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub this_should_work: foo::Type,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bar"][::std::mem::size_of::<bar>() - 4usize];
    ["Alignment of bar"][::std::mem::align_of::<bar>() - 4usize];
    [
        "Offset of field: bar::this_should_work",
    ][::std::mem::offset_of!(bar, this_should_work) - 0usize];
};
impl Default for bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn func1(
        arg1: foo::Type,
        arg2: *mut foo::Type,
        arg3: *mut *mut foo::Type,
    ) -> *mut foo::Type;
}
extern "C" {
    pub fn func2(
        arg1: foo_alias1,
        arg2: *mut foo_alias1,
        arg3: *mut *mut foo_alias1,
    ) -> *mut foo_alias1;
}
