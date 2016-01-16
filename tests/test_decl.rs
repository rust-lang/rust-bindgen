use support::assert_bind_eq;

#[test]
fn ptr_to_array() {
    assert_bind_eq("headers/decl_ptr_to_array.h", "
        extern \"C\" {
            pub static mut foo: [::std::os::raw::c_int; 1usize];
        }
    ");
}
