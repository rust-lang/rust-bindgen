#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const RTE_CACHE_LINE_MIN_SIZE: u32 = 64;
pub const RTE_CACHE_LINE_SIZE: u32 = 64;
#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy, Debug)]
pub struct rte_kni_mbuf {
    pub buf_addr: *mut ::std::os::raw::c_void,
    pub buf_physaddr: u64,
    pub pad0: [::std::os::raw::c_char; 2usize],
    ///< Start address of data in segment buffer.
    pub data_off: u16,
    pub pad1: [::std::os::raw::c_char; 2usize],
    ///< Number of segments.
    pub nb_segs: u8,
    pub pad4: [::std::os::raw::c_char; 1usize],
    ///< Offload features.
    pub ol_flags: u64,
    pub pad2: [::std::os::raw::c_char; 4usize],
    ///< Total pkt len: sum of all segment data_len.
    pub pkt_len: u32,
    ///< Amount of data in segment buffer.
    pub data_len: u16,
    pub __bindgen_padding_0: [u8; 22usize],
    pub pad3: [::std::os::raw::c_char; 8usize],
    pub pool: *mut ::std::os::raw::c_void,
    pub next: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_kni_mbuf"][::std::mem::size_of::<rte_kni_mbuf>() - 128usize];
    ["Alignment of rte_kni_mbuf"][::std::mem::align_of::<rte_kni_mbuf>() - 64usize];
    [
        "Offset of field: rte_kni_mbuf::buf_addr",
    ][::std::mem::offset_of!(rte_kni_mbuf, buf_addr) - 0usize];
    [
        "Offset of field: rte_kni_mbuf::buf_physaddr",
    ][::std::mem::offset_of!(rte_kni_mbuf, buf_physaddr) - 8usize];
    [
        "Offset of field: rte_kni_mbuf::pad0",
    ][::std::mem::offset_of!(rte_kni_mbuf, pad0) - 16usize];
    [
        "Offset of field: rte_kni_mbuf::data_off",
    ][::std::mem::offset_of!(rte_kni_mbuf, data_off) - 18usize];
    [
        "Offset of field: rte_kni_mbuf::pad1",
    ][::std::mem::offset_of!(rte_kni_mbuf, pad1) - 20usize];
    [
        "Offset of field: rte_kni_mbuf::nb_segs",
    ][::std::mem::offset_of!(rte_kni_mbuf, nb_segs) - 22usize];
    [
        "Offset of field: rte_kni_mbuf::pad4",
    ][::std::mem::offset_of!(rte_kni_mbuf, pad4) - 23usize];
    [
        "Offset of field: rte_kni_mbuf::ol_flags",
    ][::std::mem::offset_of!(rte_kni_mbuf, ol_flags) - 24usize];
    [
        "Offset of field: rte_kni_mbuf::pad2",
    ][::std::mem::offset_of!(rte_kni_mbuf, pad2) - 32usize];
    [
        "Offset of field: rte_kni_mbuf::pkt_len",
    ][::std::mem::offset_of!(rte_kni_mbuf, pkt_len) - 36usize];
    [
        "Offset of field: rte_kni_mbuf::data_len",
    ][::std::mem::offset_of!(rte_kni_mbuf, data_len) - 40usize];
    [
        "Offset of field: rte_kni_mbuf::pad3",
    ][::std::mem::offset_of!(rte_kni_mbuf, pad3) - 64usize];
    [
        "Offset of field: rte_kni_mbuf::pool",
    ][::std::mem::offset_of!(rte_kni_mbuf, pool) - 72usize];
    [
        "Offset of field: rte_kni_mbuf::next",
    ][::std::mem::offset_of!(rte_kni_mbuf, next) - 80usize];
};
impl Default for rte_kni_mbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
