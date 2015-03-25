use support::assert_bind_eq;

#[test]
fn extern_c_in_hpp() {
    assert_bind_eq("headers/extern.hpp", "
        pub type foo = extern \"C\" fn(bar: ::libc::c_int) -> ::libc::c_int;
    ");
}
