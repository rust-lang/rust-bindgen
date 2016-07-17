use support::assert_bind_eq;

#[test]
fn func_ptr() {
    assert_bind_eq(Default::default(), "headers/func_ptr.h", r#"
        pub type fn_t = ::std::option::Option<extern "C" fn()>;
        pub type fnptr_t = ::std::option::Option<extern "C" fn()>;
        pub type fntptr_t = fn_t;
        pub type fnptrptr_t = *mut ::std::option::Option<extern "C" fn()>;
        pub type fntptrptr_t = *mut fntptr_t;
        pub type fntptrtptr_t = *mut fn_t;
        extern "C" {
            pub static mut foo:
                       ::std::option::Option<extern "C" fn(x: ::std::os::raw::c_int,
                                                           y: ::std::os::raw::c_int)
                                                 -> ::std::os::raw::c_int>;
        }
        extern "C" {
            pub fn takes_argdef(proc_: ::std::option::Option<extern "C" fn()>);
            pub fn takes_argdef_ptr(proc_: ::std::option::Option<extern "C" fn()>);
            pub fn takes_argdef_ptr_ptr(proc_:
                                            *mut ::std::option::Option<extern "C" fn()>);
            pub fn takes_tdef(proc_: fn_t);
            pub fn takes_tdef_ptr(proc_: fn_t);
            pub fn takes_tdef_ptr_ptr(proc_: *mut fn_t);
            pub fn takes_tdefptr(proc_: fnptr_t);
            pub fn takes_tdefptr_ptr(proc_: *mut fnptr_t);
            pub fn takes_tdefptr_ptr_ptr(proc_: *mut *mut fnptr_t);
            pub fn takes_tdeftptr(proc_: fntptr_t);
            pub fn takes_tdeftptr_ptr(proc_: *mut fntptr_t);
            pub fn takes_tdeftptr_ptr_ptr(proc_: *mut *mut fntptr_t);
            pub fn takes_tdefptrptr(proc_: fnptrptr_t);
            pub fn takes_tdefptrptr_ptr(proc_: *mut fnptrptr_t);
            pub fn takes_tdefptrptr_ptr_ptr(proc_: *mut *mut fnptrptr_t);
            pub fn takes_tdeftptrtptr(proc_: fntptrtptr_t);
            pub fn takes_tdeftptrtptr_ptr(proc_: *mut fntptrtptr_t);
            pub fn takes_tdeftptrtptr_ptr_ptr(proc_: *mut *mut fntptrtptr_t);
        }
    "#);
}

#[test]
fn func_ptr_in_struct() {
    assert_bind_eq(Default::default(), "headers/func_ptr_in_struct.h", "
        #[derive(Copy, Clone)]
        #[repr(u32)]
        #[derive(Debug)]
        pub enum baz { TEST = 0, }
        #[repr(C)]
        #[derive(Copy, Clone)]
        #[derive(Debug)]
        pub struct Foo {
            pub bar: ::std::option::Option<
                extern \"C\" fn(x: ::std::os::raw::c_int,
                              y: ::std::os::raw::c_int) -> baz>,
        }
        impl ::std::default::Default for Foo {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
    ");
}

#[test]
fn func_proto() {
    assert_bind_eq(Default::default(), "headers/func_proto.h", "
        pub type foo = ::std::option::Option<extern \"C\" fn(bar: ::std::os::raw::c_int) -> ::std::os::raw::c_int>;
    ");
}

#[test]
fn func_no_proto() {
    assert_bind_eq(Default::default(), "headers/func_no_proto.h", "
        pub type no_proto = ::std::option::Option<extern \"C\" fn() -> ::std::os::raw::c_int>;
    ");
}

#[test]
fn with_func_ptr_arg() {
    assert_bind_eq(Default::default(), "headers/func_with_func_ptr_arg.h", "
        pub type ty = ::std::option::Option<extern \"C\" fn() -> ::std::os::raw::c_int>;
        extern \"C\" {
            pub fn foo(bar: ::std::option::Option<extern \"C\" fn()>);
            pub fn function(proc_: ty);
        }
    ");
}

#[test]
fn with_array_arg() {
    assert_bind_eq(Default::default(), "headers/func_with_array_arg.h", "
        extern \"C\" {
            pub fn f(x: *mut ::std::os::raw::c_int);
        }
    ");
}
