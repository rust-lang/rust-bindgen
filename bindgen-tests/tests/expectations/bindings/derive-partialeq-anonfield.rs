#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct rte_mbuf {
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_1,
}
#[repr(C)]
#[repr(align(1))]
#[derive(Copy, Clone)]
pub struct rte_mbuf__bindgen_ty_1 {
    pub bindgen_union_field: [u8; 0usize],
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_1>(),
        0usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_1))
    );
}
impl Default for rte_mbuf__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf>(),
        0usize,
        concat!("Size of: ", stringify!(rte_mbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_mbuf))
    );
}
impl Default for rte_mbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
