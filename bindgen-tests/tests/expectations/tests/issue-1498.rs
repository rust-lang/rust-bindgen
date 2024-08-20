#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rte_memseg {
    ///< Start physical address.
    pub phys_addr: u64,
    pub __bindgen_anon_1: rte_memseg__bindgen_ty_1,
    ///< Length of the segment.
    pub len: usize,
    ///< The pagesize of underlying memory
    pub hugepage_sz: u64,
    ///< NUMA socket ID.
    pub socket_id: i32,
    ///< Number of channels.
    pub nchannel: u32,
    ///< Number of ranks.
    pub nrank: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_memseg__bindgen_ty_1 {
    ///< Start virtual address.
    pub addr: *mut ::std::os::raw::c_void,
    ///< Makes sure addr is always 64 bits
    pub addr_64: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_memseg__bindgen_ty_1",
    ][::std::mem::size_of::<rte_memseg__bindgen_ty_1>() - 8usize];
    [
        "Alignment of rte_memseg__bindgen_ty_1",
    ][::std::mem::align_of::<rte_memseg__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: rte_memseg__bindgen_ty_1::addr",
    ][::std::mem::offset_of!(rte_memseg__bindgen_ty_1, addr) - 0usize];
    [
        "Offset of field: rte_memseg__bindgen_ty_1::addr_64",
    ][::std::mem::offset_of!(rte_memseg__bindgen_ty_1, addr_64) - 0usize];
};
impl Default for rte_memseg__bindgen_ty_1 {
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
    ["Size of rte_memseg"][::std::mem::size_of::<rte_memseg>() - 44usize];
    ["Alignment of rte_memseg"][::std::mem::align_of::<rte_memseg>() - 1usize];
    [
        "Offset of field: rte_memseg::phys_addr",
    ][::std::mem::offset_of!(rte_memseg, phys_addr) - 0usize];
    [
        "Offset of field: rte_memseg::len",
    ][::std::mem::offset_of!(rte_memseg, len) - 16usize];
    [
        "Offset of field: rte_memseg::hugepage_sz",
    ][::std::mem::offset_of!(rte_memseg, hugepage_sz) - 24usize];
    [
        "Offset of field: rte_memseg::socket_id",
    ][::std::mem::offset_of!(rte_memseg, socket_id) - 32usize];
    [
        "Offset of field: rte_memseg::nchannel",
    ][::std::mem::offset_of!(rte_memseg, nchannel) - 36usize];
    [
        "Offset of field: rte_memseg::nrank",
    ][::std::mem::offset_of!(rte_memseg, nrank) - 40usize];
};
impl Default for rte_memseg {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
