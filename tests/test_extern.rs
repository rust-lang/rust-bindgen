use support::assert_bind_eq;

#[test]
fn extern_c_in_hpp() {
    assert_bind_eq(Default::default(), "headers/extern.hpp", "
        pub type foo = extern \"C\" fn(bar: i32) -> i32;
    ");
}
