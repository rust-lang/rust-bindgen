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
#[derive(Debug, Default)]
pub struct NonCopyType {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NonCopyType() {
    const UNINIT: ::std::mem::MaybeUninit<NonCopyType> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NonCopyType>(),
        4usize,
        concat!("Size of: ", stringify!(NonCopyType))
    );
    assert_eq!(
        ::std::mem::align_of::<NonCopyType>(),
        4usize,
        concat!("Alignment of ", stringify!(NonCopyType))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NonCopyType),
            "::",
            stringify!(foo)
        )
    );
}
#[repr(C)]
pub struct WithBindgenGeneratedWrapper {
    pub non_copy_type: __BindgenUnionField<NonCopyType>,
    pub bar: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_WithBindgenGeneratedWrapper() {
    const UNINIT: ::std::mem::MaybeUninit<WithBindgenGeneratedWrapper> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<WithBindgenGeneratedWrapper>(),
        4usize,
        concat!("Size of: ", stringify!(WithBindgenGeneratedWrapper))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBindgenGeneratedWrapper>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBindgenGeneratedWrapper))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).non_copy_type) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBindgenGeneratedWrapper),
            "::",
            stringify!(non_copy_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBindgenGeneratedWrapper),
            "::",
            stringify!(bar)
        )
    );
}
impl Default for WithBindgenGeneratedWrapper {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub union WithManuallyDrop {
    pub non_copy_type: ::std::mem::ManuallyDrop<NonCopyType>,
    pub bar: ::std::mem::ManuallyDrop<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_WithManuallyDrop() {
    const UNINIT: ::std::mem::MaybeUninit<WithManuallyDrop> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<WithManuallyDrop>(),
        4usize,
        concat!("Size of: ", stringify!(WithManuallyDrop))
    );
    assert_eq!(
        ::std::mem::align_of::<WithManuallyDrop>(),
        4usize,
        concat!("Alignment of ", stringify!(WithManuallyDrop))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).non_copy_type) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithManuallyDrop),
            "::",
            stringify!(non_copy_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithManuallyDrop),
            "::",
            stringify!(bar)
        )
    );
}
impl Default for WithManuallyDrop {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct WithDefaultWrapper {
    pub non_copy_type: __BindgenUnionField<NonCopyType>,
    pub bar: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_WithDefaultWrapper() {
    const UNINIT: ::std::mem::MaybeUninit<WithDefaultWrapper> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<WithDefaultWrapper>(),
        4usize,
        concat!("Size of: ", stringify!(WithDefaultWrapper))
    );
    assert_eq!(
        ::std::mem::align_of::<WithDefaultWrapper>(),
        4usize,
        concat!("Alignment of ", stringify!(WithDefaultWrapper))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).non_copy_type) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithDefaultWrapper),
            "::",
            stringify!(non_copy_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithDefaultWrapper),
            "::",
            stringify!(bar)
        )
    );
}
impl Default for WithDefaultWrapper {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
