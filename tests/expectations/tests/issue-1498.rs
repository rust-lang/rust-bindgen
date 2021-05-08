#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type size_t = u64;
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct rte_memseg {
    ///< Start physical address.
    pub phys_addr: u64,
    pub __bindgen_anon_1: rte_memseg__bindgen_ty_1,
    ///< Length of the segment.
    pub len: size_t,
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
        unsafe {
            &(*(::std::ptr::null::<rte_memseg__bindgen_ty_1>())).addr
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_memseg__bindgen_ty_1),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_memseg__bindgen_ty_1>())).addr_64
                as *const _ as usize
        },
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
            &(*(::std::ptr::null::<rte_memseg>())).phys_addr as *const _
                as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_memseg>())).len as *const _ as usize
        },
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
            &(*(::std::ptr::null::<rte_memseg>())).hugepage_sz as *const _
                as usize
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
            &(*(::std::ptr::null::<rte_memseg>())).socket_id as *const _
                as usize
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
            &(*(::std::ptr::null::<rte_memseg>())).nchannel as *const _ as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_memseg>())).nrank as *const _ as usize
        },
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
