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
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            4usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(b))
        );
    }
    test_field_b();
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::core::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::core::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_bindgen_ty_1),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::core::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::core::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
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
    test_field_baz();
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
extern "C" {
    pub static mut bazz: _bindgen_ty_1;
}
pub type fooFunction =
    ::core::option::Option<unsafe extern "C" fn(bar: ::std::os::raw::c_int)>;
