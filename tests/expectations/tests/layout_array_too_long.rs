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
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_frag>())).ofs as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag),
            "::",
            stringify!(ofs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_frag>())).len as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ip_frag>())).mb as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag),
            "::",
            stringify!(mb)
        )
    );
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_key>())).src_dst as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_key),
            "::",
            stringify!(src_dst)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_key>())).id as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_key),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_key>())).key_len as *const _ as usize
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt__bindgen_ty_1>())).tqe_next
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt__bindgen_ty_1),
            "::",
            stringify!(tqe_next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt__bindgen_ty_1>())).tqe_prev
                as *const _ as usize
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).lru as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(lru)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).key as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).start as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).total_size as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(total_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).frag_size as *const _
                as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(frag_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).last_idx as *const _
                as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_pkt),
            "::",
            stringify!(last_idx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_pkt>())).frags as *const _ as usize
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
