#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const RTE_CACHE_LINE_MIN_SIZE: u32 = 64;
pub const RTE_CACHE_LINE_SIZE: u32 = 64;
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
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
#[test]
fn bindgen_test_layout_rte_kni_mbuf() {
    const UNINIT: ::std::mem::MaybeUninit<rte_kni_mbuf> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_kni_mbuf>(),
        128usize,
        concat!("Size of: ", stringify!(rte_kni_mbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_kni_mbuf>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_kni_mbuf))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).buf_addr) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(buf_addr)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).buf_physaddr) as usize - ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(buf_physaddr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad0) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pad0)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).data_off) as usize - ptr as usize
        },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(data_off)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad1) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nb_segs) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(nb_segs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad4) as usize - ptr as usize },
        23usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pad4)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ol_flags) as usize - ptr as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(ol_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad2) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pad2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pkt_len) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pkt_len)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).data_len) as usize - ptr as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(data_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pad3) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pad3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pool) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(pool)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_kni_mbuf),
            "::",
            stringify!(next)
        )
    );
}
impl Default for rte_kni_mbuf {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
