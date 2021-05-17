#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        56usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i8_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(i8_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.u8_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(u8_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i16_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(i16_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.u16_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        12usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(u16_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i32_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(i32_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.u32_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        20usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(u32_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i64_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        24usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(i64_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.u64_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        28usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(u64_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.i128_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo),
            "::",
            stringify!(i128_)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.u128_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo),
            "::",
            stringify!(u128_)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.isize_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo),
            "::",
            stringify!(isize_)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.usize_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo),
            "::",
            stringify!(usize_)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.f32_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        48usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(f32_))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.f64_);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        52usize,
        concat!("Offset of field: ", stringify!(Foo), "::", stringify!(f64_))
    );
}
