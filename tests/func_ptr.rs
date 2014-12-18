extern crate bindgen;
extern crate regex;
extern crate syntax;

mod util;

#[test]
fn test_func_ptr() {
    let output = util::generate_unpretty_output("tests/headers/func_ptr.h");
    assert_eq!(output, r##"extern "C" { pub static mut foo: ::std::option::Option<extern "C" fn(x: ::libc::c_int, y: ::libc::c_int) -> ::libc::c_int>; }"##);
}

#[test]
fn test_func_ptr_in_struct() {
    let output = util::generate_unpretty_output("tests/headers/func_ptr_in_struct.h");
    assert_eq!(output, r##"#[repr(C)] #[deriving(Copy)] pub struct Struct_Foo { pub bar: ::std::option::Option<extern "C" fn(x: ::libc::c_int, y: ::libc::c_int) -> Enum_baz>, } extern "C" { }"##);
}
