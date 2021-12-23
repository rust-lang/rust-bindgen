#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// Deriving PartialEq for rust unions is not supported.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ShouldNotDerivePartialEq {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShouldNotDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDerivePartialEq>(),
        4usize,
        concat!("Size of: ", stringify!(ShouldNotDerivePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDerivePartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldNotDerivePartialEq))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDerivePartialEq>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDerivePartialEq),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDerivePartialEq>())).b as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDerivePartialEq),
            "::",
            stringify!(b)
        )
    );
}
impl Default for ShouldNotDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
