#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate core;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub large_array: [::std::os::raw::c_int; 420usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::core::mem::size_of::<C>(),
        1680usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::core::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<C>())).large_array as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(large_array)
        )
    );
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::core::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::core::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::core::cmp::PartialEq for C {
    fn eq(&self, other: &C) -> bool {
        &self.large_array[..] == &other.large_array[..]
    }
}
