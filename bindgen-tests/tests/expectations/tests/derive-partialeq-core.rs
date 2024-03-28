#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern crate core;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub large_array: [::std::os::raw::c_int; 420usize],
}
#[test]
fn bindgen_test_layout_C() {
    const UNINIT: ::core::mem::MaybeUninit<C> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::core::mem::size_of::<C>(), 1680usize, "Size of C");
    assert_eq!(::core::mem::align_of::<C>(), 4usize, "Alignment of C");
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).large_array) as usize - ptr as usize },
        0usize,
        "Offset of field: C::large_array",
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
