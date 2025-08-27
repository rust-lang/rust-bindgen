#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ip_frag"][::std::mem::size_of::<ip_frag>() - 16usize];
    ["Alignment of ip_frag"][::std::mem::align_of::<ip_frag>() - 8usize];
    ["Offset of field: ip_frag::ofs"][::std::mem::offset_of!(ip_frag, ofs) - 0usize];
    ["Offset of field: ip_frag::len"][::std::mem::offset_of!(ip_frag, len) - 2usize];
    ["Offset of field: ip_frag::mb"][::std::mem::offset_of!(ip_frag, mb) - 8usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ip_frag_key"][::std::mem::size_of::<ip_frag_key>() - 40usize];
    ["Alignment of ip_frag_key"][::std::mem::align_of::<ip_frag_key>() - 8usize];
    [
        "Offset of field: ip_frag_key::src_dst",
    ][::std::mem::offset_of!(ip_frag_key, src_dst) - 0usize];
    [
        "Offset of field: ip_frag_key::id",
    ][::std::mem::offset_of!(ip_frag_key, id) - 32usize];
    [
        "Offset of field: ip_frag_key::key_len",
    ][::std::mem::offset_of!(ip_frag_key, key_len) - 36usize];
};
/** @internal Fragmented packet to reassemble.
 First two entries in the frags[] array are for the last and first fragments.*/
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ip_frag_pkt__bindgen_ty_1",
    ][::std::mem::size_of::<ip_frag_pkt__bindgen_ty_1>() - 16usize];
    [
        "Alignment of ip_frag_pkt__bindgen_ty_1",
    ][::std::mem::align_of::<ip_frag_pkt__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: ip_frag_pkt__bindgen_ty_1::tqe_next",
    ][::std::mem::offset_of!(ip_frag_pkt__bindgen_ty_1, tqe_next) - 0usize];
    [
        "Offset of field: ip_frag_pkt__bindgen_ty_1::tqe_prev",
    ][::std::mem::offset_of!(ip_frag_pkt__bindgen_ty_1, tqe_prev) - 8usize];
};
impl Default for ip_frag_pkt__bindgen_ty_1 {
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
    ["Size of ip_frag_pkt"][::std::mem::size_of::<ip_frag_pkt>() - 192usize];
    ["Alignment of ip_frag_pkt"][::std::mem::align_of::<ip_frag_pkt>() - 64usize];
    [
        "Offset of field: ip_frag_pkt::lru",
    ][::std::mem::offset_of!(ip_frag_pkt, lru) - 0usize];
    [
        "Offset of field: ip_frag_pkt::key",
    ][::std::mem::offset_of!(ip_frag_pkt, key) - 16usize];
    [
        "Offset of field: ip_frag_pkt::start",
    ][::std::mem::offset_of!(ip_frag_pkt, start) - 56usize];
    [
        "Offset of field: ip_frag_pkt::total_size",
    ][::std::mem::offset_of!(ip_frag_pkt, total_size) - 64usize];
    [
        "Offset of field: ip_frag_pkt::frag_size",
    ][::std::mem::offset_of!(ip_frag_pkt, frag_size) - 68usize];
    [
        "Offset of field: ip_frag_pkt::last_idx",
    ][::std::mem::offset_of!(ip_frag_pkt, last_idx) - 72usize];
    [
        "Offset of field: ip_frag_pkt::frags",
    ][::std::mem::offset_of!(ip_frag_pkt, frags) - 80usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ip_pkt_list"][::std::mem::size_of::<ip_pkt_list>() - 16usize];
    ["Alignment of ip_pkt_list"][::std::mem::align_of::<ip_pkt_list>() - 8usize];
    [
        "Offset of field: ip_pkt_list::tqh_first",
    ][::std::mem::offset_of!(ip_pkt_list, tqh_first) - 0usize];
    [
        "Offset of field: ip_pkt_list::tqh_last",
    ][::std::mem::offset_of!(ip_pkt_list, tqh_last) - 8usize];
};
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
#[derive(Debug, Copy, Clone)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ip_frag_tbl_stat"][::std::mem::size_of::<ip_frag_tbl_stat>() - 64usize];
    [
        "Alignment of ip_frag_tbl_stat",
    ][::std::mem::align_of::<ip_frag_tbl_stat>() - 64usize];
    [
        "Offset of field: ip_frag_tbl_stat::find_num",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, find_num) - 0usize];
    [
        "Offset of field: ip_frag_tbl_stat::add_num",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, add_num) - 8usize];
    [
        "Offset of field: ip_frag_tbl_stat::del_num",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, del_num) - 16usize];
    [
        "Offset of field: ip_frag_tbl_stat::reuse_num",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, reuse_num) - 24usize];
    [
        "Offset of field: ip_frag_tbl_stat::fail_total",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, fail_total) - 32usize];
    [
        "Offset of field: ip_frag_tbl_stat::fail_nospace",
    ][::std::mem::offset_of!(ip_frag_tbl_stat, fail_nospace) - 40usize];
};
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
#[derive(Debug)]
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
    pub __bindgen_padding_0: __BindgenOpaqueArray8<[u8; 8usize]>,
    ///< statistics counters.
    pub stat: ip_frag_tbl_stat,
    ///< hash table.
    pub pkt: __IncompleteArrayField<ip_frag_pkt>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_ip_frag_tbl"][::std::mem::size_of::<rte_ip_frag_tbl>() - 128usize];
    [
        "Alignment of rte_ip_frag_tbl",
    ][::std::mem::align_of::<rte_ip_frag_tbl>() - 64usize];
    [
        "Offset of field: rte_ip_frag_tbl::max_cycles",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, max_cycles) - 0usize];
    [
        "Offset of field: rte_ip_frag_tbl::entry_mask",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, entry_mask) - 8usize];
    [
        "Offset of field: rte_ip_frag_tbl::max_entries",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, max_entries) - 12usize];
    [
        "Offset of field: rte_ip_frag_tbl::use_entries",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, use_entries) - 16usize];
    [
        "Offset of field: rte_ip_frag_tbl::bucket_entries",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, bucket_entries) - 20usize];
    [
        "Offset of field: rte_ip_frag_tbl::nb_entries",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, nb_entries) - 24usize];
    [
        "Offset of field: rte_ip_frag_tbl::nb_buckets",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, nb_buckets) - 28usize];
    [
        "Offset of field: rte_ip_frag_tbl::last",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, last) - 32usize];
    [
        "Offset of field: rte_ip_frag_tbl::lru",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, lru) - 40usize];
    [
        "Offset of field: rte_ip_frag_tbl::stat",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, stat) - 64usize];
    [
        "Offset of field: rte_ip_frag_tbl::pkt",
    ][::std::mem::offset_of!(rte_ip_frag_tbl, pkt) - 128usize];
};
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
