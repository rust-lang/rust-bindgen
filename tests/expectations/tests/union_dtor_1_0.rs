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
#[derive(Debug, Default)]
pub struct UnionWithDtor {
    pub mFoo: __BindgenUnionField<::std::os::raw::c_int>,
    pub mBar: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_UnionWithDtor() {
    const UNINIT: ::std::mem::MaybeUninit<UnionWithDtor> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<UnionWithDtor>(),
        8usize,
        concat!("Size of: ", stringify!(UnionWithDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<UnionWithDtor>(),
        8usize,
        concat!("Alignment of ", stringify!(UnionWithDtor))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mFoo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UnionWithDtor),
            "::",
            stringify!(mFoo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mBar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UnionWithDtor),
            "::",
            stringify!(mBar)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN13UnionWithDtorD1Ev"]
    pub fn UnionWithDtor_UnionWithDtor_destructor(this: *mut UnionWithDtor);
}
impl UnionWithDtor {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        UnionWithDtor_UnionWithDtor_destructor(self)
    }
}
