#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct Test_Interface__bindgen_vtable {
    pub Virtual_Method: unsafe extern "C" fn(this: *mut Test_Interface),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Test_Interface {
    pub vtable_: *const Test_Interface__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_Test_Interface() {
    assert_eq!(
        ::std::mem::size_of::<Test_Interface>(),
        8usize,
        concat!("Size of: ", stringify!(Test_Interface)),
    );
    assert_eq!(
        ::std::mem::align_of::<Test_Interface>(),
        8usize,
        concat!("Alignment of ", stringify!(Test_Interface)),
    );
}
impl Default for Test_Interface {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
