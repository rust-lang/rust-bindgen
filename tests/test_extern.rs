use support::assert_bind_eq;

#[test]
fn extern_c_in_hpp() {
    assert_bind_eq(Default::default(), "headers/extern.hpp", "
        pub type foo = extern \"C\" fn(bar: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    ");
}
