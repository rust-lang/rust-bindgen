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
#[derive(Copy)]
pub struct WithBigArray {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<[::std::os::raw::c_int; 33usize]>,
    pub bindgen_union_field: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    const UNINIT: ::std::mem::MaybeUninit<WithBigArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<WithBigArray>(), 132usize, "Size of WithBigArray");
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        "Alignment of WithBigArray",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigArray::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigArray::b",
    );
}
impl Clone for WithBigArray {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for WithBigArray {
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
pub struct WithBigArray2 {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<[::std::os::raw::c_char; 33usize]>,
    pub bindgen_union_field: [u32; 9usize],
}
#[test]
fn bindgen_test_layout_WithBigArray2() {
    const UNINIT: ::std::mem::MaybeUninit<WithBigArray2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::std::mem::size_of::<WithBigArray2>(), 36usize, "Size of WithBigArray2");
    assert_eq!(
        ::std::mem::align_of::<WithBigArray2>(),
        4usize,
        "Alignment of WithBigArray2",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigArray2::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigArray2::b",
    );
}
impl Clone for WithBigArray2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct WithBigMember {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<WithBigArray>,
    pub bindgen_union_field: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigMember() {
    const UNINIT: ::std::mem::MaybeUninit<WithBigMember> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<WithBigMember>(),
        132usize,
        "Size of WithBigMember",
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigMember>(),
        4usize,
        "Alignment of WithBigMember",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigMember::a",
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        "Offset of field: WithBigMember::b",
    );
}
impl Clone for WithBigMember {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for WithBigMember {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
