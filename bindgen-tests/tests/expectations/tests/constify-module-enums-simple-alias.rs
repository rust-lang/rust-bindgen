#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod Foo {
    pub type Type = ::std::os::raw::c_int;
    pub const Variant1: Type = 0;
    pub const Variant2: Type = 1;
    pub const Variant3: Type = 2;
}
pub use self::Foo::Type as Foo_alias1;
pub use self::Foo_alias1 as Foo_alias2;
pub use self::Foo_alias2 as Foo_alias3;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub baz1: Foo::Type,
    pub baz2: Foo_alias1,
    pub baz3: Foo_alias2,
    pub baz4: Foo_alias3,
    pub baz_ptr1: *mut Foo::Type,
    pub baz_ptr2: *mut Foo_alias1,
    pub baz_ptr3: *mut Foo_alias2,
    pub baz_ptr4: *mut Foo_alias3,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 48usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::baz1"][::std::mem::offset_of!(Bar, baz1) - 0usize];
    ["Offset of field: Bar::baz2"][::std::mem::offset_of!(Bar, baz2) - 4usize];
    ["Offset of field: Bar::baz3"][::std::mem::offset_of!(Bar, baz3) - 8usize];
    ["Offset of field: Bar::baz4"][::std::mem::offset_of!(Bar, baz4) - 12usize];
    ["Offset of field: Bar::baz_ptr1"][::std::mem::offset_of!(Bar, baz_ptr1) - 16usize];
    ["Offset of field: Bar::baz_ptr2"][::std::mem::offset_of!(Bar, baz_ptr2) - 24usize];
    ["Offset of field: Bar::baz_ptr3"][::std::mem::offset_of!(Bar, baz_ptr3) - 32usize];
    ["Offset of field: Bar::baz_ptr4"][::std::mem::offset_of!(Bar, baz_ptr4) - 40usize];
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
