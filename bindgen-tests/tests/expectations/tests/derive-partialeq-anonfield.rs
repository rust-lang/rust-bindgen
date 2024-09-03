#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mbuf__bindgen_ty_1",
    ][::std::mem::size_of::<rte_mbuf__bindgen_ty_1>() - 0usize];
    [
        "Alignment of rte_mbuf__bindgen_ty_1",
    ][::std::mem::align_of::<rte_mbuf__bindgen_ty_1>() - 1usize];
};
impl Default for rte_mbuf__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_mbuf"][::std::mem::size_of::<rte_mbuf>() - 0usize];
    ["Alignment of rte_mbuf"][::std::mem::align_of::<rte_mbuf>() - 64usize];
};
impl Default for rte_mbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
