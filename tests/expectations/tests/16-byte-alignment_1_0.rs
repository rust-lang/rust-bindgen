#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_ipv4_tuple {
    pub src_addr: u32,
    pub dst_addr: u32,
    pub __bindgen_anon_1: rte_ipv4_tuple__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_ipv4_tuple__bindgen_ty_1 {
    pub __bindgen_anon_1:
        __BindgenUnionField<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>,
    pub sctp_tag: __BindgenUnionField<u32>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(
            )))
            .dport as *const _ as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(
            )))
            .sport as *const _ as usize
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
impl Clone for rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ipv4_tuple__bindgen_ty_1>())).sctp_tag
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv4_tuple__bindgen_ty_1),
            "::",
            stringify!(sctp_tag)
        )
    );
}
impl Clone for rte_ipv4_tuple__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv4_tuple>())).src_addr as *const _
                as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv4_tuple>())).dst_addr as *const _
                as usize
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
impl Clone for rte_ipv4_tuple {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_ipv6_tuple {
    pub src_addr: [u8; 16usize],
    pub dst_addr: [u8; 16usize],
    pub __bindgen_anon_1: rte_ipv6_tuple__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_ipv6_tuple__bindgen_ty_1 {
    pub __bindgen_anon_1:
        __BindgenUnionField<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>,
    pub sctp_tag: __BindgenUnionField<u32>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(
            )))
            .dport as *const _ as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(
            )))
            .sport as *const _ as usize
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
impl Clone for rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
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
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_ipv6_tuple__bindgen_ty_1>())).sctp_tag
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_ipv6_tuple__bindgen_ty_1),
            "::",
            stringify!(sctp_tag)
        )
    );
}
impl Clone for rte_ipv6_tuple__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv6_tuple>())).src_addr as *const _
                as usize
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
        unsafe {
            &(*(::std::ptr::null::<rte_ipv6_tuple>())).dst_addr as *const _
                as usize
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
impl Clone for rte_ipv6_tuple {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct rte_thash_tuple {
    pub v4: __BindgenUnionField<rte_ipv4_tuple>,
    pub v6: __BindgenUnionField<rte_ipv6_tuple>,
    pub bindgen_union_field: [u8; 48usize],
}
#[test]
fn bindgen_test_layout_rte_thash_tuple() {
    assert_eq!(
        ::std::mem::size_of::<rte_thash_tuple>(),
        48usize,
        concat!("Size of: ", stringify!(rte_thash_tuple))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_thash_tuple>())).v4 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_thash_tuple),
            "::",
            stringify!(v4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rte_thash_tuple>())).v6 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rte_thash_tuple),
            "::",
            stringify!(v6)
        )
    );
}
impl Clone for rte_thash_tuple {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for rte_thash_tuple {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
