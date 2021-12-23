#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
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
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Default, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_pkt_list {
    pub tqh_first: *mut ip_frag_pkt,
    pub tqh_last: *mut *mut ip_frag_pkt,
}
#[test]
fn bindgen_test_layout_ip_pkt_list() {
    assert_eq!(
        ::std::mem::size_of::<ip_pkt_list>(),
        16usize,
        concat!("Size of: ", stringify!(ip_pkt_list))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_pkt_list>(),
        8usize,
        concat!("Alignment of ", stringify!(ip_pkt_list))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_pkt_list>())).tqh_first as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_pkt_list),
            "::",
            stringify!(tqh_first)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_pkt_list>())).tqh_last as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_pkt_list),
            "::",
            stringify!(tqh_last)
        )
    );
}
impl Default for ip_pkt_list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// fragmentation table statistics
#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct ip_frag_tbl_stat {
    ///< total # of find/insert attempts.
    pub find_num: u64,
    ///< # of add ops.
    pub add_num: u64,
    ///< # of del ops.
    pub del_num: u64,
    ///< # of reuse (del/add) ops.
    pub reuse_num: u64,
    ///< total # of add failures.
    pub fail_total: u64,
    ///< # of 'no space' add failures.
    pub fail_nospace: u64,
}
#[test]
fn bindgen_test_layout_ip_frag_tbl_stat() {
    assert_eq!(
        ::std::mem::size_of::<ip_frag_tbl_stat>(),
        64usize,
        concat!("Size of: ", stringify!(ip_frag_tbl_stat))
    );
    assert_eq!(
        ::std::mem::align_of::<ip_frag_tbl_stat>(),
        64usize,
        concat!("Alignment of ", stringify!(ip_frag_tbl_stat))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).find_num as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(find_num)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).add_num as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(add_num)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).del_num as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(del_num)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).reuse_num as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(reuse_num)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).fail_total as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(fail_total)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ip_frag_tbl_stat>())).fail_nospace
                as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ip_frag_tbl_stat),
            "::",
            stringify!(fail_nospace)
        )
    );
}
impl Default for ip_frag_tbl_stat {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// fragmentation table
#[repr(C)]
#[repr(align(64))]
pub struct rte_ip_frag_tbl {
    ///< ttl for table entries.
    pub max_cycles: u64,
    ///< hash value mask.
    pub entry_mask: u32,
    ///< max entries allowed.
    pub max_entries: u32,
    ///< entries in use.
    pub use_entries: u32,
    ///< hash assocaitivity.
    pub bucket_entries: u32,
    ///< total size of the table.
    pub nb_entries: u32,
    ///< num of associativity lines.
    pub nb_buckets: u32,
    ///< last used entry.
    pub last: *mut ip_frag_pkt,
    ///< LRU list for table entries.
    pub lru: ip_pkt_list,
    pub __bindgen_padding_0: u64,
    ///< statistics counters.
    pub stat: ip_frag_tbl_stat,
    ///< hash table.
    pub pkt: __IncompleteArrayField<ip_frag_pkt>,
}
#[test]
fn bindgen_test_layout_rte_ip_frag_tbl() {
    assert_eq!(
        ::std::mem::size_of::<rte_ip_frag_tbl>(),
        128usize,
        concat!("Size of: ", stringify!(rte_ip_frag_tbl))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ip_frag_tbl>(),
        64usize,
        concat!("Alignment of ", stringify!(rte_ip_frag_tbl))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).max_cycles as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(max_cycles)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).entry_mask as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(entry_mask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).max_entries as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(max_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).use_entries as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(use_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).bucket_entries
                as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(bucket_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).nb_entries as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(nb_entries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).nb_buckets as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(nb_buckets)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).last as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(last)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).lru as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(lru)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).stat as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(stat)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ip_frag_tbl>())).pkt as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ip_frag_tbl),
            "::",
            stringify!(pkt)
        )
    );
}
impl Default for rte_ip_frag_tbl {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
///< fragment mbuf
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_mbuf {
    pub _address: u8,
}
