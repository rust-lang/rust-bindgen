#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        *self
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
    const UNINIT: ::core::mem::MaybeUninit<foo> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::core::mem::size_of::<foo>(), 16usize, "Size of foo");
    assert_eq!(::core::mem::align_of::<foo>(), 8usize, "Alignment of foo");
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        "Offset of field: foo::a",
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        4usize,
        "Offset of field: foo::b",
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        8usize,
        "Offset of field: foo::bar",
    );
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for foo {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::core::mem::uninitialized();
            ::core::ptr::write_bytes(&mut s, 0, 1);
            s
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
    const UNINIT: ::core::mem::MaybeUninit<_bindgen_ty_1> = ::core::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(::core::mem::size_of::<_bindgen_ty_1>(), 8usize, "Size of _bindgen_ty_1");
    assert_eq!(
        ::core::mem::align_of::<_bindgen_ty_1>(),
        8usize,
        "Alignment of _bindgen_ty_1",
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        "Offset of field: _bindgen_ty_1::bar",
    );
    assert_eq!(
        unsafe { ::core::ptr::addr_of!((*ptr).baz) as usize - ptr as usize },
        0usize,
        "Offset of field: _bindgen_ty_1::baz",
    );
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
extern "C" {
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction = ::core::option::Option<
    unsafe extern "C" fn(bar: ::std::os::raw::c_int),
>;
