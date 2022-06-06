#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: *const *const *mut *const ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
