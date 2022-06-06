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
    fn test_field_tqh_first() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_pkt_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tqh_first) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_pkt_list),
                "::",
                stringify!(tqh_first)
            )
        );
    }
    test_field_tqh_first();
    fn test_field_tqh_last() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ip_pkt_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tqh_last) as usize - ptr as usize
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
    test_field_tqh_last();
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
    fn test_field_find_num() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).find_num) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_tbl_stat),
                "::",
                stringify!(find_num)
            )
        );
    }
    test_field_find_num();
    fn test_field_add_num() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).add_num) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_tbl_stat),
                "::",
                stringify!(add_num)
            )
        );
    }
    test_field_add_num();
    fn test_field_del_num() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).del_num) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_tbl_stat),
                "::",
                stringify!(del_num)
            )
        );
    }
    test_field_del_num();
    fn test_field_reuse_num() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reuse_num) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_tbl_stat),
                "::",
                stringify!(reuse_num)
            )
        );
    }
    test_field_reuse_num();
    fn test_field_fail_total() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fail_total) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(ip_frag_tbl_stat),
                "::",
                stringify!(fail_total)
            )
        );
    }
    test_field_fail_total();
    fn test_field_fail_nospace() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ip_frag_tbl_stat>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fail_nospace) as usize -
                    ptr as usize
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
    test_field_fail_nospace();
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
    fn test_field_max_cycles() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).max_cycles) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(max_cycles)
            )
        );
    }
    test_field_max_cycles();
    fn test_field_entry_mask() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).entry_mask) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(entry_mask)
            )
        );
    }
    test_field_entry_mask();
    fn test_field_max_entries() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).max_entries) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(max_entries)
            )
        );
    }
    test_field_max_entries();
    fn test_field_use_entries() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).use_entries) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(use_entries)
            )
        );
    }
    test_field_use_entries();
    fn test_field_bucket_entries() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bucket_entries) as usize -
                    ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(bucket_entries)
            )
        );
    }
    test_field_bucket_entries();
    fn test_field_nb_entries() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).nb_entries) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(nb_entries)
            )
        );
    }
    test_field_nb_entries();
    fn test_field_nb_buckets() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).nb_buckets) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(nb_buckets)
            )
        );
    }
    test_field_nb_buckets();
    fn test_field_last() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).last) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(last)
            )
        );
    }
    test_field_last();
    fn test_field_lru() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lru) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(lru)
            )
        );
    }
    test_field_lru();
    fn test_field_stat() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).stat) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ip_frag_tbl),
                "::",
                stringify!(stat)
            )
        );
    }
    test_field_stat();
    fn test_field_pkt() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_ip_frag_tbl>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).pkt) as usize - ptr as usize
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
    test_field_pkt();
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
