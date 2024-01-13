#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
/// Bizarrely enough, this should *not* get an `_address` field.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ZeroSizedArray {
    pub arr: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_ZeroSizedArray() {
    const UNINIT: ::std::mem::MaybeUninit<ZeroSizedArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ZeroSizedArray>(),
        0usize,
        concat!("Size of: ", stringify!(ZeroSizedArray)),
    );
    assert_eq!(
        ::std::mem::align_of::<ZeroSizedArray>(),
        1usize,
        concat!("Alignment of ", stringify!(ZeroSizedArray)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arr) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ZeroSizedArray), "::", stringify!(arr)),
    );
}
/// And nor should this get an `_address` field.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ContainsZeroSizedArray {
    pub zsa: ZeroSizedArray,
}
#[test]
fn bindgen_test_layout_ContainsZeroSizedArray() {
    const UNINIT: ::std::mem::MaybeUninit<ContainsZeroSizedArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ContainsZeroSizedArray>(),
        0usize,
        concat!("Size of: ", stringify!(ContainsZeroSizedArray)),
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsZeroSizedArray>(),
        1usize,
        concat!("Alignment of ", stringify!(ContainsZeroSizedArray)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).zsa) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsZeroSizedArray),
            "::",
            stringify!(zsa),
        ),
    );
}
/** Inheriting from ZeroSizedArray shouldn't cause an `_address` to be inserted
 either.*/
#[repr(C)]
#[derive(Debug, Default)]
pub struct InheritsZeroSizedArray {
    pub _base: ZeroSizedArray,
}
#[test]
fn bindgen_test_layout_InheritsZeroSizedArray() {
    assert_eq!(
        ::std::mem::size_of::<InheritsZeroSizedArray>(),
        0usize,
        concat!("Size of: ", stringify!(InheritsZeroSizedArray)),
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsZeroSizedArray>(),
        1usize,
        concat!("Alignment of ", stringify!(InheritsZeroSizedArray)),
    );
}
/// And this should not get an `_address` field either.
#[repr(C)]
#[derive(Debug, Default)]
pub struct DynamicallySizedArray {
    pub arr: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_DynamicallySizedArray() {
    const UNINIT: ::std::mem::MaybeUninit<DynamicallySizedArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DynamicallySizedArray>(),
        0usize,
        concat!("Size of: ", stringify!(DynamicallySizedArray)),
    );
    assert_eq!(
        ::std::mem::align_of::<DynamicallySizedArray>(),
        1usize,
        concat!("Alignment of ", stringify!(DynamicallySizedArray)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).arr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DynamicallySizedArray),
            "::",
            stringify!(arr),
        ),
    );
}
/// No `_address` field here either.
#[repr(C)]
#[derive(Debug, Default)]
pub struct ContainsDynamicallySizedArray {
    pub dsa: DynamicallySizedArray,
}
#[test]
fn bindgen_test_layout_ContainsDynamicallySizedArray() {
    const UNINIT: ::std::mem::MaybeUninit<ContainsDynamicallySizedArray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ContainsDynamicallySizedArray>(),
        0usize,
        concat!("Size of: ", stringify!(ContainsDynamicallySizedArray)),
    );
    assert_eq!(
        ::std::mem::align_of::<ContainsDynamicallySizedArray>(),
        1usize,
        concat!("Alignment of ", stringify!(ContainsDynamicallySizedArray)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dsa) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContainsDynamicallySizedArray),
            "::",
            stringify!(dsa),
        ),
    );
}
