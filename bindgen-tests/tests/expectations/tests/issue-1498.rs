#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

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
#[test]
fn bindgen_test_layout_rte_memseg__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<rte_memseg__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_memseg__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(rte_memseg__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_memseg__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_memseg__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg__bindgen_ty_1),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).addr_64) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg__bindgen_ty_1),
            "::",
            stringify!(addr_64)
        )
    );
}
impl Default for rte_memseg__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_memseg() {
    const UNINIT: ::std::mem::MaybeUninit<rte_memseg> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_memseg>(),
        44usize,
        concat!("Size of: ", stringify!(rte_memseg))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_memseg>(),
        1usize,
        concat!("Alignment of ", stringify!(rte_memseg))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).phys_addr) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(phys_addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).hugepage_sz) as usize - ptr as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(hugepage_sz)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).socket_id) as usize - ptr as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(socket_id)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).nchannel) as usize - ptr as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(nchannel)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nrank) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg),
            "::",
            stringify!(nrank)
        )
    );
}
impl Default for rte_memseg {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
