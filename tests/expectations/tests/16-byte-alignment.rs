#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rte_ipv4_tuple {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub __bindgen_anon_1: rte_ipv4_tuple__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_ipv4_tuple__bindgen_ty_1 {
    pub __bindgen_anon_1: rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1,
    pub sctp_tag: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1 {
    pub dport: u16,
    pub sport: u16,
}
#[test]
fn bindgen_test_layout_rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr = &struct_instance
                as *const rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.dport);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(dport)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr = &struct_instance
                as *const rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.sport);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(sport)
        )
    );
}
#[test]
fn bindgen_test_layout_rte_ipv4_tuple__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(rte_ipv4_tuple__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ipv4_tuple__bindgen_ty_1))
    );
}
impl Default for rte_ipv4_tuple__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_ipv4_tuple() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple>(),
        12usize,
        concat!("Size of: ", stringify!(rte_ipv4_tuple))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ipv4_tuple))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<rte_ipv4_tuple>() };
            let struct_ptr = &struct_instance as *const rte_ipv4_tuple;
            let field_ptr = std::ptr::addr_of!(struct_instance.src_addr);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv4_tuple),
            "::",
            stringify!(src_addr)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<rte_ipv4_tuple>() };
            let struct_ptr = &struct_instance as *const rte_ipv4_tuple;
            let field_ptr = std::ptr::addr_of!(struct_instance.dst_addr);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv4_tuple),
            "::",
            stringify!(dst_addr)
        )
    );
}
impl Default for rte_ipv4_tuple {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rte_ipv6_tuple {
    pub src_addr: [u8; 16usize],
    pub dst_addr: [u8; 16usize],
    pub __bindgen_anon_1: rte_ipv6_tuple__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rte_ipv6_tuple__bindgen_ty_1 {
    pub __bindgen_anon_1: rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1,
    pub sctp_tag: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1 {
    pub dport: u16,
    pub sport: u16,
}
#[test]
fn bindgen_test_layout_rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr = &struct_instance
                as *const rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.dport);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(dport)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr = &struct_instance
                as *const rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.sport);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(sport)
        )
    );
}
#[test]
fn bindgen_test_layout_rte_ipv6_tuple__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(rte_ipv6_tuple__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ipv6_tuple__bindgen_ty_1))
    );
}
impl Default for rte_ipv6_tuple__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_rte_ipv6_tuple() {
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple>(),
        36usize,
        concat!("Size of: ", stringify!(rte_ipv6_tuple))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ipv6_tuple))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<rte_ipv6_tuple>() };
            let struct_ptr = &struct_instance as *const rte_ipv6_tuple;
            let field_ptr = std::ptr::addr_of!(struct_instance.src_addr);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv6_tuple),
            "::",
            stringify!(src_addr)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<rte_ipv6_tuple>() };
            let struct_ptr = &struct_instance as *const rte_ipv6_tuple;
            let field_ptr = std::ptr::addr_of!(struct_instance.dst_addr);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv6_tuple),
            "::",
            stringify!(dst_addr)
        )
    );
}
impl Default for rte_ipv6_tuple {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union rte_thash_tuple {
    pub v4: rte_ipv4_tuple,
    pub v6: rte_ipv6_tuple,
}
#[test]
fn bindgen_test_layout_rte_thash_tuple() {
    assert_eq!(
        ::std::mem::size_of::<rte_thash_tuple>(),
        48usize,
        concat!("Size of: ", stringify!(rte_thash_tuple))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_thash_tuple>(),
        16usize,
        concat!("Alignment of ", stringify!(rte_thash_tuple))
    );
}
impl Default for rte_thash_tuple {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
