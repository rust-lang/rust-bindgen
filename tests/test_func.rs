#[test]
fn func_ptr() {
    assert_bind_eq!("headers/func_ptr.h", cx,
        quote_item!(cx,
            extern "C" {
                pub static mut foo: ::std::option::Option<
                    extern "C" fn(x: ::libc::c_int,
                                  y: ::libc::c_int) -> ::libc::c_int>;
            }
        )
    );
}

#[test]
fn func_ptr_in_struct() {
    assert_bind_eq!("headers/func_ptr_in_struct.h", cx,
        quote_item!(cx,
            #[repr(C)]
            #[derive(Copy)]
            pub struct Struct_Foo {
                pub bar: ::std::option::Option<
                    extern "C" fn(x: ::libc::c_int,
                                  y: ::libc::c_int) -> Enum_baz>,
            }
        ),
        quote_item!(cx,
            impl ::std::default::Default for Struct_Foo {
                fn default() -> Struct_Foo { unsafe { ::std::mem::zeroed() } }
            }
        )
    );
}

#[test]
fn func_proto() {
    assert_bind_eq!("headers/func_proto.h", cx,
        quote_item!(cx,
            pub type foo = extern "C" fn(bar: ::libc::c_int) -> ::libc::c_int;
        ));
}

#[test]
fn with_func_ptr_arg() {
    assert_bind_eq!("headers/func_with_func_ptr_arg.h", cx,
        quote_item!(cx,
            extern "C" {
                pub fn foo(bar: ::std::option::Option<extern "C" fn() -> () >) -> ();
            }
        ));
}

#[test]
fn with_array_arg() {
    assert_bind_eq!("headers/func_with_array_arg.h", cx,
        quote_item!(cx,
            extern "C" {
                pub fn f(x: *mut ::libc::c_int) -> ();
            }
        )
    );
}
