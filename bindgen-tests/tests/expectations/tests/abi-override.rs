#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
unsafe extern "fastcall" {
    pub fn foo();
}
unsafe extern "stdcall" {
    pub fn bar();
}
unsafe extern "C" {
    pub fn baz();
}
unsafe extern "system" {
    pub fn qux();
}
pub type boo = ::std::option::Option<unsafe extern "efiapi" fn()>;
pub type foobar = ::std::option::Option<unsafe extern "efiapi" fn(boo: boo)>;
