#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    pub fn foo(bar: ::std::option::Option<unsafe extern "C" fn()>);
}
extern "C" {
    pub fn bar(
        one: ::std::option::Option<
            unsafe extern "C" fn(
                a: ::std::os::raw::c_int,
                b: ::std::os::raw::c_int,
            ),
        >,
        two: ::std::option::Option<
            unsafe extern "C" fn(
                c: ::std::os::raw::c_int,
                d: ::std::os::raw::c_int,
            ),
        >,
    );
}
