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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.phys_addr);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.len);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.hugepage_sz);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.socket_id);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.nchannel);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<rte_memseg>() };
            let struct_ptr = &struct_instance as *const rte_memseg;
            let field_ptr = std::ptr::addr_of!(struct_instance.nrank);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
