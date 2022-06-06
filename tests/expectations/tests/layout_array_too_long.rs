#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const RTE_CACHE_LINE_SIZE: u32 = 64;
pub const RTE_LIBRTE_IP_FRAG_MAX_FRAG: u32 = 4;
pub const IP_LAST_FRAG_IDX: _bindgen_ty_1 = _bindgen_ty_1::IP_LAST_FRAG_IDX;
pub const IP_FIRST_FRAG_IDX: _bindgen_ty_1 = _bindgen_ty_1::IP_FIRST_FRAG_IDX;
pub const IP_MIN_FRAG_NUM: _bindgen_ty_1 = _bindgen_ty_1::IP_MIN_FRAG_NUM;
pub const IP_MAX_FRAG_NUM: _bindgen_ty_1 = _bindgen_ty_1::IP_MAX_FRAG_NUM;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    ///< index of last fragment
    IP_LAST_FRAG_IDX = 0,
    ///< index of first fragment
    IP_FIRST_FRAG_IDX = 1,
    ///< minimum number of fragments
    IP_MIN_FRAG_NUM = 2,
    IP_MAX_FRAG_NUM = 4,
}
/// @internal fragmented mbuf
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ip_frag {
    ///< offset into the packet
    pub ofs: u16,
    ///< length of fragment
    pub len: u16,
    ///< fragment mbuf
    pub mb: *mut rte_mbuf,
}
#[test]
fn bindgen_test_layout_ip_frag() {
    assert_eq!(
        ::std::mem::size_of::<ip_frag>(),
        16usize,
        concat!("Size of: ", stringify!(ip_frag))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_frag>(),
        8usize,
        concat!("Alignment of ", stringify!(ip_frag))
    );
    fn test_field_ofs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ofs) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag),
                "::",
                stringify!(ofs)
            )
        );
    }
    test_field_ofs();
    fn test_field_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag),
                "::",
                stringify!(len)
            )
        );
    }
    test_field_len();
    fn test_field_mb() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mb) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag),
                "::",
                stringify!(mb)
            )
        );
    }
    test_field_mb();
}
impl Default for ip_frag {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// @internal <src addr, dst_addr, id> to uniquely indetify fragmented datagram.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ip_frag_key {
    ///< src address, first 8 bytes used for IPv4
    pub src_dst: [u64; 4usize],
    ///< dst address
    pub id: u32,
    ///< src/dst key length
    pub key_len: u32,
}
#[test]
fn bindgen_test_layout_ip_frag_key() {
    assert_eq!(
        ::std::mem::size_of::<ip_frag_key>(),
        40usize,
        concat!("Size of: ", stringify!(ip_frag_key))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_frag_key>(),
        8usize,
        concat!("Alignment of ", stringify!(ip_frag_key))
    );
    fn test_field_src_dst() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_key>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).src_dst) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_key),
                "::",
                stringify!(src_dst)
            )
        );
    }
    test_field_src_dst();
    fn test_field_id() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_key>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_key),
                "::",
                stringify!(id)
            )
        );
    }
    test_field_id();
    fn test_field_key_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_key>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key_len) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_key),
                "::",
                stringify!(key_len)
            )
        );
    }
    test_field_key_len();
}
/// @internal Fragmented packet to reassemble.
/// First two entries in the frags[] array are for the last and first fragments.
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct ip_frag_pkt {
    ///< LRU list
    pub lru: ip_frag_pkt__bindgen_ty_1,
    ///< fragmentation key
    pub key: ip_frag_key,
    ///< creation timestamp
    pub start: u64,
    ///< expected reassembled size
    pub total_size: u32,
    ///< size of fragments received
    pub frag_size: u32,
    ///< index of next entry to fill
    pub last_idx: u32,
    ///< fragments
    pub frags: [ip_frag; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct ip_frag_pkt__bindgen_ty_1 {
    pub tqe_next: *mut ip_frag_pkt,
    pub tqe_prev: *mut *mut ip_frag_pkt,
}
#[test]
fn bindgen_test_layout_ip_frag_pkt__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ip_frag_pkt__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ip_frag_pkt__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_frag_pkt__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ip_frag_pkt__bindgen_ty_1))
    );
    fn test_field_tqe_next() {
        assert_eq!(
            unsafe {
                let uninit = :: std :: mem :: MaybeUninit :: < ip_frag_pkt__bindgen_ty_1 > :: uninit () ;
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tqe_next) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt__bindgen_ty_1),
                "::",
                stringify!(tqe_next)
            )
        );
    }
    test_field_tqe_next();
    fn test_field_tqe_prev() {
        assert_eq!(
            unsafe {
                let uninit = :: std :: mem :: MaybeUninit :: < ip_frag_pkt__bindgen_ty_1 > :: uninit () ;
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tqe_prev) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt__bindgen_ty_1),
                "::",
                stringify!(tqe_prev)
            )
        );
    }
    test_field_tqe_prev();
}
impl Default for ip_frag_pkt__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_ip_frag_pkt() {
    assert_eq!(
        ::std::mem::size_of::<ip_frag_pkt>(),
        192usize,
        concat!("Size of: ", stringify!(ip_frag_pkt))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_frag_pkt>(),
        64usize,
        concat!("Alignment of ", stringify!(ip_frag_pkt))
    );
    fn test_field_lru() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lru) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(lru)
            )
        );
    }
    test_field_lru();
    fn test_field_key() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).key) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(key)
            )
        );
    }
    test_field_key();
    fn test_field_start() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).start) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(start)
            )
        );
    }
    test_field_start();
    fn test_field_total_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).total_size) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(total_size)
            )
        );
    }
    test_field_total_size();
    fn test_field_frag_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).frag_size) as usize - ptr as usize
            },
            68usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(frag_size)
            )
        );
    }
    test_field_frag_size();
    fn test_field_last_idx() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).last_idx) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(last_idx)
            )
        );
    }
    test_field_last_idx();
    fn test_field_frags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_frag_pkt>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).frags) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_pkt),
                "::",
                stringify!(frags)
            )
        );
    }
    test_field_frags();
}
impl Default for ip_frag_pkt {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ip_frag_pkt {
    fn eq(&self, other: &ip_frag_pkt) -> bool {
        self.lru == other.lru &&
            self.key == other.key &&
            self.start == other.start &&
            self.total_size == other.total_size &&
            self.frag_size == other.frag_size &&
            self.last_idx == other.last_idx &&
            self.frags == other.frags
    }
}
///< fragment mbuf
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mbuf {
    pub _address: u8,
}
