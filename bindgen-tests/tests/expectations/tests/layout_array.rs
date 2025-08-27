#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
pub const RTE_CACHE_LINE_SIZE: u32 = 64;
pub const RTE_MEMPOOL_OPS_NAMESIZE: u32 = 32;
pub const RTE_MEMPOOL_MAX_OPS_IDX: u32 = 16;
pub const RTE_HEAP_NUM_FREELISTS: u32 = 13;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rte_mempool {
    _unused: [u8; 0],
}
/** Prototype for implementation specific data provisioning function.

 The function should provide the implementation specific memory for
 for use by the other mempool ops functions in a given mempool ops struct.
 E.g. the default ops provides an instance of the rte_ring for this purpose.
 it will most likely point to a different type of data structure, and
 will be transparent to the application programmer.
 This function should set mp->pool_data.*/
pub type rte_mempool_alloc_t = ::std::option::Option<
    unsafe extern "C" fn(mp: *mut rte_mempool) -> ::std::os::raw::c_int,
>;
/// Free the opaque private data pointed to by mp->pool_data pointer.
pub type rte_mempool_free_t = ::std::option::Option<
    unsafe extern "C" fn(mp: *mut rte_mempool),
>;
/// Enqueue an object into the external pool.
pub type rte_mempool_enqueue_t = ::std::option::Option<
    unsafe extern "C" fn(
        mp: *mut rte_mempool,
        obj_table: *const *mut ::std::os::raw::c_void,
        n: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
/// Dequeue an object from the external pool.
pub type rte_mempool_dequeue_t = ::std::option::Option<
    unsafe extern "C" fn(
        mp: *mut rte_mempool,
        obj_table: *mut *mut ::std::os::raw::c_void,
        n: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
/// Return the number of available objects in the external pool.
pub type rte_mempool_get_count = ::std::option::Option<
    unsafe extern "C" fn(mp: *const rte_mempool) -> ::std::os::raw::c_uint,
>;
/// Structure defining mempool operations structure
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mempool_ops {
    ///< Name of mempool ops struct.
    pub name: [::std::os::raw::c_char; 32usize],
    ///< Allocate private data.
    pub alloc: rte_mempool_alloc_t,
    ///< Free the external pool.
    pub free: rte_mempool_free_t,
    ///< Enqueue an object.
    pub enqueue: rte_mempool_enqueue_t,
    ///< Dequeue an object.
    pub dequeue: rte_mempool_dequeue_t,
    ///< Get qty of available objs.
    pub get_count: rte_mempool_get_count,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_mempool_ops"][::std::mem::size_of::<rte_mempool_ops>() - 128usize];
    [
        "Alignment of rte_mempool_ops",
    ][::std::mem::align_of::<rte_mempool_ops>() - 64usize];
    [
        "Offset of field: rte_mempool_ops::name",
    ][::std::mem::offset_of!(rte_mempool_ops, name) - 0usize];
    [
        "Offset of field: rte_mempool_ops::alloc",
    ][::std::mem::offset_of!(rte_mempool_ops, alloc) - 32usize];
    [
        "Offset of field: rte_mempool_ops::free",
    ][::std::mem::offset_of!(rte_mempool_ops, free) - 40usize];
    [
        "Offset of field: rte_mempool_ops::enqueue",
    ][::std::mem::offset_of!(rte_mempool_ops, enqueue) - 48usize];
    [
        "Offset of field: rte_mempool_ops::dequeue",
    ][::std::mem::offset_of!(rte_mempool_ops, dequeue) - 56usize];
    [
        "Offset of field: rte_mempool_ops::get_count",
    ][::std::mem::offset_of!(rte_mempool_ops, get_count) - 64usize];
};
impl Default for rte_mempool_ops {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// The rte_spinlock_t type.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_spinlock_t {
    ///< lock status 0 = unlocked, 1 = locked
    pub locked: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_spinlock_t"][::std::mem::size_of::<rte_spinlock_t>() - 4usize];
    ["Alignment of rte_spinlock_t"][::std::mem::align_of::<rte_spinlock_t>() - 4usize];
    [
        "Offset of field: rte_spinlock_t::locked",
    ][::std::mem::offset_of!(rte_spinlock_t, locked) - 0usize];
};
/** Structure storing the table of registered ops structs, each of which contain
 the function pointers for the mempool ops functions.
 Each process has its own storage for this ops struct array so that
 the mempools can be shared across primary and secondary processes.
 The indices used to access the array are valid across processes, whereas
 any function pointers stored directly in the mempool struct would not be.
 This results in us simply having "ops_index" in the mempool struct.*/
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_mempool_ops_table {
    ///< Spinlock for add/delete.
    pub sl: rte_spinlock_t,
    ///< Number of used ops structs in the table.
    pub num_ops: u32,
    pub __bindgen_padding_0: __BindgenOpaqueArray8<[u8; 56usize]>,
    /// Storage for all possible ops structs.
    pub ops: [rte_mempool_ops; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_mempool_ops_table",
    ][::std::mem::size_of::<rte_mempool_ops_table>() - 2112usize];
    [
        "Alignment of rte_mempool_ops_table",
    ][::std::mem::align_of::<rte_mempool_ops_table>() - 64usize];
    [
        "Offset of field: rte_mempool_ops_table::sl",
    ][::std::mem::offset_of!(rte_mempool_ops_table, sl) - 0usize];
    [
        "Offset of field: rte_mempool_ops_table::num_ops",
    ][::std::mem::offset_of!(rte_mempool_ops_table, num_ops) - 4usize];
    [
        "Offset of field: rte_mempool_ops_table::ops",
    ][::std::mem::offset_of!(rte_mempool_ops_table, ops) - 64usize];
};
impl Default for rte_mempool_ops_table {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// Structure to hold malloc heap
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct malloc_heap {
    pub lock: rte_spinlock_t,
    pub free_head: [malloc_heap__bindgen_ty_1; 13usize],
    pub alloc_count: ::std::os::raw::c_uint,
    pub total_size: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct malloc_heap__bindgen_ty_1 {
    pub lh_first: *mut malloc_elem,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of malloc_heap__bindgen_ty_1",
    ][::std::mem::size_of::<malloc_heap__bindgen_ty_1>() - 8usize];
    [
        "Alignment of malloc_heap__bindgen_ty_1",
    ][::std::mem::align_of::<malloc_heap__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: malloc_heap__bindgen_ty_1::lh_first",
    ][::std::mem::offset_of!(malloc_heap__bindgen_ty_1, lh_first) - 0usize];
};
impl Default for malloc_heap__bindgen_ty_1 {
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
    ["Size of malloc_heap"][::std::mem::size_of::<malloc_heap>() - 128usize];
    ["Alignment of malloc_heap"][::std::mem::align_of::<malloc_heap>() - 64usize];
    [
        "Offset of field: malloc_heap::lock",
    ][::std::mem::offset_of!(malloc_heap, lock) - 0usize];
    [
        "Offset of field: malloc_heap::free_head",
    ][::std::mem::offset_of!(malloc_heap, free_head) - 8usize];
    [
        "Offset of field: malloc_heap::alloc_count",
    ][::std::mem::offset_of!(malloc_heap, alloc_count) - 112usize];
    [
        "Offset of field: malloc_heap::total_size",
    ][::std::mem::offset_of!(malloc_heap, total_size) - 120usize];
};
impl Default for malloc_heap {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct malloc_elem {
    pub _address: u8,
}
