#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type i8_ = i8;
pub type u8_ = u8;
pub type i16_ = i16;
pub type u16_ = u16;
pub type i32_ = i32;
pub type u32_ = u32;
pub type i64_ = i64;
pub type u64_ = u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub i8_: ::std::os::raw::c_int,
    pub u8_: ::std::os::raw::c_int,
    pub i16_: ::std::os::raw::c_int,
    pub u16_: ::std::os::raw::c_int,
    pub i32_: ::std::os::raw::c_int,
    pub u32_: ::std::os::raw::c_int,
    pub i64_: ::std::os::raw::c_int,
    pub u64_: ::std::os::raw::c_int,
    pub i128_: ::std::os::raw::c_int,
    pub u128_: ::std::os::raw::c_int,
    pub isize_: ::std::os::raw::c_int,
    pub usize_: ::std::os::raw::c_int,
    pub f32_: ::std::os::raw::c_int,
    pub f64_: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 56usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 4usize, "Alignment of Foo");
    assert!(::std::mem::offset_of!(Foo, i8_) == 0usize, "Offset of field: Foo::i8_");
    assert!(::std::mem::offset_of!(Foo, u8_) == 4usize, "Offset of field: Foo::u8_");
    assert!(::std::mem::offset_of!(Foo, i16_) == 8usize, "Offset of field: Foo::i16_");
    assert!(::std::mem::offset_of!(Foo, u16_) == 12usize, "Offset of field: Foo::u16_");
    assert!(::std::mem::offset_of!(Foo, i32_) == 16usize, "Offset of field: Foo::i32_");
    assert!(::std::mem::offset_of!(Foo, u32_) == 20usize, "Offset of field: Foo::u32_");
    assert!(::std::mem::offset_of!(Foo, i64_) == 24usize, "Offset of field: Foo::i64_");
    assert!(::std::mem::offset_of!(Foo, u64_) == 28usize, "Offset of field: Foo::u64_");
    assert!(
        ::std::mem::offset_of!(Foo, i128_) == 32usize,
        "Offset of field: Foo::i128_",
    );
    assert!(
        ::std::mem::offset_of!(Foo, u128_) == 36usize,
        "Offset of field: Foo::u128_",
    );
    assert!(
        ::std::mem::offset_of!(Foo, isize_) == 40usize,
        "Offset of field: Foo::isize_",
    );
    assert!(
        ::std::mem::offset_of!(Foo, usize_) == 44usize,
        "Offset of field: Foo::usize_",
    );
    assert!(::std::mem::offset_of!(Foo, f32_) == 48usize, "Offset of field: Foo::f32_");
    assert!(::std::mem::offset_of!(Foo, f64_) == 52usize, "Offset of field: Foo::f64_");
};
