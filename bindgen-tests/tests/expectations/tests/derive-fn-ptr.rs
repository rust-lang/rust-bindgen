#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 8usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 8usize, "Alignment of Foo");
    assert!(
        ::std::mem::offset_of!(Foo, callback) == 0usize,
        "Offset of field: Foo::callback",
    );
};
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
const _: () = {
    assert!(::std::mem::size_of::<Bar>() == 8usize, "Size of Bar");
    assert!(::std::mem::align_of::<Bar>() == 8usize, "Alignment of Bar");
    assert!(
        ::std::mem::offset_of!(Bar, callback) == 0usize,
        "Offset of field: Bar::callback",
    );
};
