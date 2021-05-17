#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Test {
    pub foo: ::std::os::raw::c_int,
    pub bar: f32,
}
pub const Test_T_NONE: Test__bindgen_ty_1 = Test__bindgen_ty_1::T_NONE;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Test__bindgen_ty_1 {
    T_NONE = 0,
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        8usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        4usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.foo);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.bar);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(bar))
    );
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Baz {
    Foo = 0,
    Bar = 1,
}
