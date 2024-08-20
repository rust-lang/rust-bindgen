#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod one_Foo {
    pub type Type = ::std::os::raw::c_int;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub baz1: one_Foo::Type,
    pub baz2: *mut one_Foo::Type,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 16usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::baz1"][::std::mem::offset_of!(Bar, baz1) - 0usize];
    ["Offset of field: Bar::baz2"][::std::mem::offset_of!(Bar, baz2) - 8usize];
};
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
