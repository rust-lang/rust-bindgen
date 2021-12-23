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
#[derive(Copy)]
pub struct WithBigArray {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<[::std::os::raw::c_int; 33usize]>,
    pub bindgen_union_field: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray),
            "::",
            stringify!(b)
        )
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
    assert_eq!(
        ::std::mem::size_of::<WithBigArray2>(),
        36usize,
        concat!("Size of: ", stringify!(WithBigArray2))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray2>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray2>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray2),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigArray2>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray2),
            "::",
            stringify!(b)
        )
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
    assert_eq!(
        ::std::mem::size_of::<WithBigMember>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigMember))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigMember>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigMember))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigMember>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigMember),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<WithBigMember>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigMember),
            "::",
            stringify!(b)
        )
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
