#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate core;

#[repr(C)]
pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::core::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::core::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::core::mem::transmute(self)
    }
}
impl<T> ::core::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::core::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::core::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::core::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::core::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::core::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Copy, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
    pub bar: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::core::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::core::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).b as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<foo>())).bar as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(bar))
    );
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
struct Box_foo {
    ptr: *mut ::core::ffi::c_void,
}
impl Box_foo {}
impl Drop for Box_foo {
    fn drop(&mut self) {
        unsafe {
            ::core::alloc::dealloc(
                self.ptr as *mut u8,
                ::core::alloc::Layout::from_size_align(16usize, 8usize)
                    .unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct _bindgen_ty_1 {
    pub bar: __BindgenUnionField<::std::os::raw::c_int>,
    pub baz: __BindgenUnionField<::std::os::raw::c_long>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<_bindgen_ty_1>())).bar as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(bar)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<_bindgen_ty_1>())).baz as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(baz)
        )
    );
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
struct Box__bindgen_ty_1 {
    ptr: *mut ::core::ffi::c_void,
}
impl Box__bindgen_ty_1 {}
impl Drop for Box__bindgen_ty_1 {
    fn drop(&mut self) {
        unsafe {
            ::core::alloc::dealloc(
                self.ptr as *mut u8,
                ::core::alloc::Layout::from_size_align(8usize, 8usize).unwrap(),
            );
        }
    }
}
extern "C" {
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction =
    ::core::option::Option<unsafe extern "C" fn(bar: ::std::os::raw::c_int)>;
