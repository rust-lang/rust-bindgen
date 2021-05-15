#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        48usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz1))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz2))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz3);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz3))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        12usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(baz4))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz_ptr1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(baz_ptr1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz_ptr2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(baz_ptr2)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz_ptr3);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(baz_ptr3)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.baz_ptr4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(baz_ptr4)
        )
    );
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
