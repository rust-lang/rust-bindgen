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
    fn test_field_addr() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_memseg__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).addr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_memseg__bindgen_ty_1),
                "::",
                stringify!(addr)
            )
        );
    }
    test_field_addr();
    fn test_field_addr_64() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_memseg__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).addr_64) as usize - ptr as usize
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
    test_field_addr_64();
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
    fn test_field_phys_addr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
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
    }
    test_field_phys_addr();
    fn test_field_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_memseg),
                "::",
                stringify!(len)
            )
        );
    }
    test_field_len();
    fn test_field_hugepage_sz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
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
    }
    test_field_hugepage_sz();
    fn test_field_socket_id() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
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
    }
    test_field_socket_id();
    fn test_field_nchannel() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
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
    }
    test_field_nchannel();
    fn test_field_nrank() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_memseg>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).nrank) as usize - ptr as usize
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
    test_field_nrank();
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
