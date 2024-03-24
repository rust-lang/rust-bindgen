#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct Instance {
    pub val: [u32; 50usize],
}
#[test]
fn bindgen_test_layout_Instance() {
    const UNINIT: ::std::mem::MaybeUninit<Instance> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Instance>(), 200usize, "Size of Instance");
    assert_eq!(::std::mem::align_of::<Instance>(), 4usize, "Alignment of Instance");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        "Offset of field: Instance::val",
    );
}
impl Default for Instance {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "Instance {{ val: opaque }}")
    }
}
