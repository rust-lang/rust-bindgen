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
    const UNINIT: ::std::mem::MaybeUninit<Foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe {
            ::std::ptr::addr_of!((*ptr).callback) as usize - ptr as usize
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
    const UNINIT: ::std::mem::MaybeUninit<Bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe {
            ::std::ptr::addr_of!((*ptr).callback) as usize - ptr as usize
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
