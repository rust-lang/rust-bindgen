#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type my_fun_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: ::std::os::raw::c_int,
        arg8: ::std::os::raw::c_int,
        arg9: ::std::os::raw::c_int,
        arg10: ::std::os::raw::c_int,
        arg11: ::std::os::raw::c_int,
        arg12: ::std::os::raw::c_int,
        arg13: ::std::os::raw::c_int,
        arg14: ::std::os::raw::c_int,
        arg15: ::std::os::raw::c_int,
        arg16: ::std::os::raw::c_int,
    ),
>;
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct Foo {
    pub callback: my_fun_t,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        8usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        8usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Foo>() };
            let struct_ptr = &struct_instance as *const Foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.callback);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Foo),
            "::",
            stringify!(callback)
        )
    );
}
pub type my_fun2_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
        arg7: ::std::os::raw::c_int,
        arg8: ::std::os::raw::c_int,
        arg9: ::std::os::raw::c_int,
        arg10: ::std::os::raw::c_int,
        arg11: ::std::os::raw::c_int,
        arg12: ::std::os::raw::c_int,
    ),
>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Bar {
    pub callback: my_fun2_t,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
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
            let field_ptr = std::ptr::addr_of!(struct_instance.callback);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(callback)
        )
    );
}
