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
    pub const fn new() -> Self {
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
#[derive(Clone, Copy)]
pub struct ShouldDeriveStruct {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_int,
    pub nested: ShouldDeriveStruct__bindgen_ty_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShouldDeriveStruct__bindgen_ty_1 {
    pub c: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShouldDeriveStruct__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveStruct__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(ShouldDeriveStruct__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveStruct__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(ShouldDeriveStruct__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveStruct__bindgen_ty_1>())).c
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveStruct__bindgen_ty_1),
            "::",
            stringify!(c)
        )
    );
}
#[test]
fn bindgen_test_layout_ShouldDeriveStruct() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveStruct>(),
        12usize,
        concat!("Size of: ", stringify!(ShouldDeriveStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveStruct>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldDeriveStruct))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveStruct>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveStruct),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveStruct>())).b as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveStruct),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveStruct>())).nested as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveStruct),
            "::",
            stringify!(nested)
        )
    );
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShouldDeriveUnion {
    pub a: __BindgenUnionField<::std::os::raw::c_char>,
    pub b: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_ShouldDeriveUnion() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDeriveUnion>(),
        4usize,
        concat!("Size of: ", stringify!(ShouldDeriveUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDeriveUnion>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldDeriveUnion))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveUnion>())).a as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveUnion),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDeriveUnion>())).b as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDeriveUnion),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
pub struct ShouldNotDeriveStruct {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShouldNotDeriveStruct() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDeriveStruct>(),
        8usize,
        concat!("Size of: ", stringify!(ShouldNotDeriveStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDeriveStruct>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldNotDeriveStruct))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDeriveStruct>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDeriveStruct),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldNotDeriveStruct>())).b as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldNotDeriveStruct),
            "::",
            stringify!(b)
        )
    );
}
