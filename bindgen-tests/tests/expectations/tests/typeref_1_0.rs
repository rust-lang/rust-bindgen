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
pub struct mozilla_FragmentOrURL {
    pub mIsLocalRef: bool,
}
#[test]
fn bindgen_test_layout_mozilla_FragmentOrURL() {
    const UNINIT: ::std::mem::MaybeUninit<mozilla_FragmentOrURL> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<mozilla_FragmentOrURL>(),
        1usize,
        "Size of mozilla_FragmentOrURL",
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_FragmentOrURL>(),
        1usize,
        "Alignment of mozilla_FragmentOrURL",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mIsLocalRef) as usize - ptr as usize },
        0usize,
        "Offset of field: mozilla_FragmentOrURL::mIsLocalRef",
    );
}
impl Clone for mozilla_FragmentOrURL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct mozilla_Position {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_mozilla_Position() {
    assert_eq!(
        ::std::mem::size_of::<mozilla_Position>(),
        1usize,
        "Size of mozilla_Position",
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_Position>(),
        1usize,
        "Alignment of mozilla_Position",
    );
}
impl Clone for mozilla_Position {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_StyleShapeSource {
    pub __bindgen_anon_1: mozilla_StyleShapeSource__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct mozilla_StyleShapeSource__bindgen_ty_1 {
    pub mPosition: __BindgenUnionField<*mut mozilla_Position>,
    pub mFragmentOrURL: __BindgenUnionField<*mut mozilla_FragmentOrURL>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct Bar {
    pub mFoo: *mut nsFoo,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<Bar>(), 8usize, "Size of Bar");
    assert_eq!(::std::mem::align_of::<Bar>(), 8usize, "Alignment of Bar");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mFoo) as usize - ptr as usize },
        0usize,
        "Offset of field: Bar::mFoo",
    );
}
impl Clone for Bar {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct nsFoo {
    pub mBar: mozilla_StyleShapeSource,
}
#[test]
fn bindgen_test_layout_nsFoo() {
    const UNINIT: ::std::mem::MaybeUninit<nsFoo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<nsFoo>(), 8usize, "Size of nsFoo");
    assert_eq!(::std::mem::align_of::<nsFoo>(), 8usize, "Alignment of nsFoo");
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBar) as usize - ptr as usize },
        0usize,
        "Offset of field: nsFoo::mBar",
    );
}
impl Clone for nsFoo {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn __bindgen_test_layout_mozilla_StyleShapeSource_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<mozilla_StyleShapeSource>(),
        8usize,
        "Size of template specialization: mozilla_StyleShapeSource_open0_int_close0",
    );
    assert_eq!(
        ::std::mem::align_of::<mozilla_StyleShapeSource>(),
        8usize,
        "Align of template specialization: mozilla_StyleShapeSource_open0_int_close0",
    );
}
