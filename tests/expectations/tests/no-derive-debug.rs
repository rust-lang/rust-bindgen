#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct foo {
    bar: ::std::os::raw::c_int,
}

/// bar should compile. It will normally derive debug, but our blocklist of foo
/// and replacement for another type that doesn't implement it would prevent it
/// from building if --no-derive-debug didn't work.
#[repr(C)]
pub struct bar {
    pub foo: foo,
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_bar() {
    assert_eq!(
        ::std::mem::size_of::<bar>(),
        8usize,
        concat!("Size of: ", stringify!(bar))
    );
    assert_eq!(
        ::std::mem::align_of::<bar>(),
        4usize,
        concat!("Alignment of ", stringify!(bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).foo as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(bar), "::", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<bar>())).baz as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(bar), "::", stringify!(baz))
    );
}
impl Default for bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
