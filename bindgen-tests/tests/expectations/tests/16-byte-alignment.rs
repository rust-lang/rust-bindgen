#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Alignment of rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>() - 2usize];
    [
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1::dport",
    ][::std::mem::offset_of!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1, dport)
        - 0usize];
    [
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1::sport",
    ][::std::mem::offset_of!(rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1, sport)
        - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_ipv4_tuple__bindgen_ty_1",
    ][::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1>() - 4usize];
    [
        "Alignment of rte_ipv4_tuple__bindgen_ty_1",
    ][::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1::sctp_tag",
    ][::std::mem::offset_of!(rte_ipv4_tuple__bindgen_ty_1, sctp_tag) - 0usize];
};
impl Default for rte_ipv4_tuple__bindgen_ty_1 {
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
    ["Size of rte_ipv4_tuple"][::std::mem::size_of::<rte_ipv4_tuple>() - 12usize];
    ["Alignment of rte_ipv4_tuple"][::std::mem::align_of::<rte_ipv4_tuple>() - 4usize];
    [
        "Offset of field: rte_ipv4_tuple::src_addr",
    ][::std::mem::offset_of!(rte_ipv4_tuple, src_addr) - 0usize];
    [
        "Offset of field: rte_ipv4_tuple::dst_addr",
    ][::std::mem::offset_of!(rte_ipv4_tuple, dst_addr) - 4usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    [
        "Alignment of rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1",
    ][::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>() - 2usize];
    [
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1::dport",
    ][::std::mem::offset_of!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1, dport)
        - 0usize];
    [
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1::sport",
    ][::std::mem::offset_of!(rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1, sport)
        - 2usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of rte_ipv6_tuple__bindgen_ty_1",
    ][::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1>() - 4usize];
    [
        "Alignment of rte_ipv6_tuple__bindgen_ty_1",
    ][::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1::sctp_tag",
    ][::std::mem::offset_of!(rte_ipv6_tuple__bindgen_ty_1, sctp_tag) - 0usize];
};
impl Default for rte_ipv6_tuple__bindgen_ty_1 {
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
    ["Size of rte_ipv6_tuple"][::std::mem::size_of::<rte_ipv6_tuple>() - 36usize];
    ["Alignment of rte_ipv6_tuple"][::std::mem::align_of::<rte_ipv6_tuple>() - 4usize];
    [
        "Offset of field: rte_ipv6_tuple::src_addr",
    ][::std::mem::offset_of!(rte_ipv6_tuple, src_addr) - 0usize];
    [
        "Offset of field: rte_ipv6_tuple::dst_addr",
    ][::std::mem::offset_of!(rte_ipv6_tuple, dst_addr) - 16usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of rte_thash_tuple"][::std::mem::size_of::<rte_thash_tuple>() - 48usize];
    [
        "Alignment of rte_thash_tuple",
    ][::std::mem::align_of::<rte_thash_tuple>() - 16usize];
    [
        "Offset of field: rte_thash_tuple::v4",
    ][::std::mem::offset_of!(rte_thash_tuple, v4) - 0usize];
    [
        "Offset of field: rte_thash_tuple::v6",
    ][::std::mem::offset_of!(rte_thash_tuple, v6) - 0usize];
};
impl Default for rte_thash_tuple {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
