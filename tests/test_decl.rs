use support::assert_bind_eq;

#[test]
fn ptr_to_array() {
    assert_bind_eq(Default::default(), "headers/decl_ptr_to_array.h", "
        extern \"C\" {
            pub static mut foo: [i32; 1usize];
        }
    ");
}
