#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        *self
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
    pub __bindgen_anon_1: __BindgenUnionField<
        rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1,
    >,
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
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        "Size of rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        "Alignment of rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dport) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1::dport",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sport) as usize - ptr as usize },
        2usize,
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1::sport",
    );
}
impl Clone for rte_ipv4_tuple__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_ipv4_tuple__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv4_tuple__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple__bindgen_ty_1>(),
        4usize,
        "Size of rte_ipv4_tuple__bindgen_ty_1",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple__bindgen_ty_1>(),
        4usize,
        "Alignment of rte_ipv4_tuple__bindgen_ty_1",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sctp_tag) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv4_tuple__bindgen_ty_1::sctp_tag",
    );
}
impl Clone for rte_ipv4_tuple__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_ipv4_tuple() {
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv4_tuple> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv4_tuple>(),
        12usize,
        "Size of rte_ipv4_tuple",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv4_tuple>(),
        4usize,
        "Alignment of rte_ipv4_tuple",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_addr) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv4_tuple::src_addr",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_addr) as usize - ptr as usize },
        4usize,
        "Offset of field: rte_ipv4_tuple::dst_addr",
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
    pub __bindgen_anon_1: __BindgenUnionField<
        rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1,
    >,
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
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        "Size of rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        "Alignment of rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dport) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1::dport",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sport) as usize - ptr as usize },
        2usize,
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1::sport",
    );
}
impl Clone for rte_ipv6_tuple__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_ipv6_tuple__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv6_tuple__bindgen_ty_1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple__bindgen_ty_1>(),
        4usize,
        "Size of rte_ipv6_tuple__bindgen_ty_1",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple__bindgen_ty_1>(),
        4usize,
        "Alignment of rte_ipv6_tuple__bindgen_ty_1",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sctp_tag) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv6_tuple__bindgen_ty_1::sctp_tag",
    );
}
impl Clone for rte_ipv6_tuple__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_ipv6_tuple() {
    const UNINIT: ::std::mem::MaybeUninit<rte_ipv6_tuple> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_ipv6_tuple>(),
        36usize,
        "Size of rte_ipv6_tuple",
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ipv6_tuple>(),
        4usize,
        "Alignment of rte_ipv6_tuple",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).src_addr) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_ipv6_tuple::src_addr",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dst_addr) as usize - ptr as usize },
        16usize,
        "Offset of field: rte_ipv6_tuple::dst_addr",
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
    const UNINIT: ::std::mem::MaybeUninit<rte_thash_tuple> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rte_thash_tuple>(),
        48usize,
        "Size of rte_thash_tuple",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v4) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_thash_tuple::v4",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v6) as usize - ptr as usize },
        0usize,
        "Offset of field: rte_thash_tuple::v6",
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
