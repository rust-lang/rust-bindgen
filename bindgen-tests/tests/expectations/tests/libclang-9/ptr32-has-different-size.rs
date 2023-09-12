#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TEST_STRUCT {
    pub ptr_32bit: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_TEST_STRUCT() {
    const UNINIT: ::std::mem::MaybeUninit<TEST_STRUCT> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<TEST_STRUCT>(),
        8usize,
        concat!("Size of: ", stringify!(TEST_STRUCT)),
    );
    assert_eq!(
        ::std::mem::align_of::<TEST_STRUCT>(),
        8usize,
        concat!("Alignment of ", stringify!(TEST_STRUCT)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ptr_32bit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TEST_STRUCT),
            "::",
            stringify!(ptr_32bit),
        ),
    );
}
impl Default for TEST_STRUCT {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type TEST = TEST_STRUCT;
