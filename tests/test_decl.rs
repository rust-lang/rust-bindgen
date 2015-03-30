#[test]
fn ptr_to_array() {
    assert_bind_eq!("headers/decl_ptr_to_array.h", cx,
        quote_item!(cx,
            extern "C" {
                pub static mut foo: [::libc::c_int; 1usize];
            }
        )
    );

}
