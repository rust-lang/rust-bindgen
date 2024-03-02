#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#![feature(ptr_metadata, layout_for_ptr)]
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
#[repr(C)]
#[derive(Debug, Default)]
pub struct flexarray<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub count: ::std::os::raw::c_int,
    pub data: FAM,
}
#[test]
fn bindgen_test_layout_flexarray() {
    const UNINIT: ::std::mem::MaybeUninit<flexarray> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<flexarray>(),
        4usize,
        concat!("Size of: ", stringify!(flexarray)),
    );
    assert_eq!(
        ::std::mem::align_of::<flexarray>(),
        4usize,
        concat!("Alignment of ", stringify!(flexarray)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(flexarray), "::", stringify!(count)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(flexarray), "::", stringify!(data)),
    );
}
impl flexarray<[::std::os::raw::c_int]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(::std::ptr::null(), len);
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    /// Construct a DST for `#canonical_ident` from a thin
    /// pointer.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr<'a>(
        ptr: *const flexarray<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> &'a Self {
        let ptr: *const Self = ::std::ptr::from_raw_parts(ptr as *const (), len);
        &*ptr
    }
    /// Construct a mutable DST for `#canonical_ident` from
    /// a thin pointer. This is `MaybeUninit` to allow for
    /// initialization.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr_mut<'a>(
        ptr: *mut flexarray<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'a mut Self> {
        let ptr: *mut Self = ::std::ptr::from_raw_parts_mut(ptr as *mut (), len);
        ::std::mem::MaybeUninit::new(&mut *ptr)
    }
}
impl flexarray<[::std::os::raw::c_int; 0]> {
    /// Turn a sized reference for `#canonical_ident` into
    /// DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref(&self, len: usize) -> &flexarray<[::std::os::raw::c_int]> {
        unsafe { flexarray::<[::std::os::raw::c_int]>::from_ptr(self, len) }
    }
    /// Turn a mutable sized reference for
    /// `#canonical_ident` into DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray<[::std::os::raw::c_int]> {
        unsafe {
            flexarray::<[::std::os::raw::c_int]>::from_ptr_mut(self, len).assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct flexarray_zero<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub count: ::std::os::raw::c_int,
    pub data: FAM,
}
#[test]
fn bindgen_test_layout_flexarray_zero() {
    const UNINIT: ::std::mem::MaybeUninit<flexarray_zero> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<flexarray_zero>(),
        4usize,
        concat!("Size of: ", stringify!(flexarray_zero)),
    );
    assert_eq!(
        ::std::mem::align_of::<flexarray_zero>(),
        4usize,
        concat!("Alignment of ", stringify!(flexarray_zero)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(flexarray_zero), "::", stringify!(count)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(flexarray_zero), "::", stringify!(data)),
    );
}
impl flexarray_zero<[::std::os::raw::c_int]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(::std::ptr::null(), len);
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    /// Construct a DST for `#canonical_ident` from a thin
    /// pointer.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr<'a>(
        ptr: *const flexarray_zero<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> &'a Self {
        let ptr: *const Self = ::std::ptr::from_raw_parts(ptr as *const (), len);
        &*ptr
    }
    /// Construct a mutable DST for `#canonical_ident` from
    /// a thin pointer. This is `MaybeUninit` to allow for
    /// initialization.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr_mut<'a>(
        ptr: *mut flexarray_zero<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'a mut Self> {
        let ptr: *mut Self = ::std::ptr::from_raw_parts_mut(ptr as *mut (), len);
        ::std::mem::MaybeUninit::new(&mut *ptr)
    }
}
impl flexarray_zero<[::std::os::raw::c_int; 0]> {
    /// Turn a sized reference for `#canonical_ident` into
    /// DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref(
        &self,
        len: usize,
    ) -> &flexarray_zero<[::std::os::raw::c_int]> {
        unsafe { flexarray_zero::<[::std::os::raw::c_int]>::from_ptr(self, len) }
    }
    /// Turn a mutable sized reference for
    /// `#canonical_ident` into DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_zero<[::std::os::raw::c_int]> {
        unsafe {
            flexarray_zero::<[::std::os::raw::c_int]>::from_ptr_mut(self, len)
                .assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct flexarray_template<T, FAM: ?Sized = [T; 0]> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub count: ::std::os::raw::c_int,
    pub data: FAM,
}
impl<T> flexarray_template<T, [T]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(::std::ptr::null(), len);
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    /// Construct a DST for `#canonical_ident` from a thin
    /// pointer.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr<'a>(
        ptr: *const flexarray_template<T, [T; 0]>,
        len: usize,
    ) -> &'a Self {
        let ptr: *const Self = ::std::ptr::from_raw_parts(ptr as *const (), len);
        &*ptr
    }
    /// Construct a mutable DST for `#canonical_ident` from
    /// a thin pointer. This is `MaybeUninit` to allow for
    /// initialization.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr_mut<'a>(
        ptr: *mut flexarray_template<T, [T; 0]>,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'a mut Self> {
        let ptr: *mut Self = ::std::ptr::from_raw_parts_mut(ptr as *mut (), len);
        ::std::mem::MaybeUninit::new(&mut *ptr)
    }
}
impl<T> flexarray_template<T, [T; 0]> {
    /// Turn a sized reference for `#canonical_ident` into
    /// DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref(&self, len: usize) -> &flexarray_template<T, [T]> {
        unsafe { flexarray_template::<T, [T]>::from_ptr(self, len) }
    }
    /// Turn a mutable sized reference for
    /// `#canonical_ident` into DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_template<T, [T]> {
        unsafe { flexarray_template::<T, [T]>::from_ptr_mut(self, len).assume_init() }
    }
}
impl<T> Default for flexarray_template<T, [T; 0]> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flexarray_ref {
    pub things: *mut flexarray,
}
#[test]
fn bindgen_test_layout_flexarray_ref() {
    const UNINIT: ::std::mem::MaybeUninit<flexarray_ref> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<flexarray_ref>(),
        8usize,
        concat!("Size of: ", stringify!(flexarray_ref)),
    );
    assert_eq!(
        ::std::mem::align_of::<flexarray_ref>(),
        8usize,
        concat!("Alignment of ", stringify!(flexarray_ref)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).things) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(flexarray_ref), "::", stringify!(things)),
    );
}
impl Default for flexarray_ref {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct flexarray_bogus_zero_fam<FAM: ?Sized = [::std::os::raw::c_char; 0]> {
    pub count: ::std::os::raw::c_int,
    pub data1: __IncompleteArrayField<::std::os::raw::c_int>,
    pub data2: FAM,
}
#[test]
fn bindgen_test_layout_flexarray_bogus_zero_fam() {
    const UNINIT: ::std::mem::MaybeUninit<flexarray_bogus_zero_fam> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<flexarray_bogus_zero_fam>(),
        4usize,
        concat!("Size of: ", stringify!(flexarray_bogus_zero_fam)),
    );
    assert_eq!(
        ::std::mem::align_of::<flexarray_bogus_zero_fam>(),
        4usize,
        concat!("Alignment of ", stringify!(flexarray_bogus_zero_fam)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flexarray_bogus_zero_fam),
            "::",
            stringify!(count),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data1) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(flexarray_bogus_zero_fam),
            "::",
            stringify!(data1),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data2) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(flexarray_bogus_zero_fam),
            "::",
            stringify!(data2),
        ),
    );
}
impl flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(::std::ptr::null(), len);
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    /// Construct a DST for `#canonical_ident` from a thin
    /// pointer.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr<'a>(
        ptr: *const flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>,
        len: usize,
    ) -> &'a Self {
        let ptr: *const Self = ::std::ptr::from_raw_parts(ptr as *const (), len);
        &*ptr
    }
    /// Construct a mutable DST for `#canonical_ident` from
    /// a thin pointer. This is `MaybeUninit` to allow for
    /// initialization.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr_mut<'a>(
        ptr: *mut flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'a mut Self> {
        let ptr: *mut Self = ::std::ptr::from_raw_parts_mut(ptr as *mut (), len);
        ::std::mem::MaybeUninit::new(&mut *ptr)
    }
}
impl flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]> {
    /// Turn a sized reference for `#canonical_ident` into
    /// DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref(
        &self,
        len: usize,
    ) -> &flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
        unsafe {
            flexarray_bogus_zero_fam::<[::std::os::raw::c_char]>::from_ptr(self, len)
        }
    }
    /// Turn a mutable sized reference for
    /// `#canonical_ident` into DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
        unsafe {
            flexarray_bogus_zero_fam::<[::std::os::raw::c_char]>::from_ptr_mut(self, len)
                .assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(128))]
#[derive(Debug)]
pub struct flexarray_align<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub count: ::std::os::raw::c_int,
    pub data: FAM,
}
#[test]
fn bindgen_test_layout_flexarray_align() {
    const UNINIT: ::std::mem::MaybeUninit<flexarray_align> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<flexarray_align>(),
        128usize,
        concat!("Size of: ", stringify!(flexarray_align)),
    );
    assert_eq!(
        ::std::mem::align_of::<flexarray_align>(),
        128usize,
        concat!("Alignment of ", stringify!(flexarray_align)),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(flexarray_align),
            "::",
            stringify!(count),
        ),
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(flexarray_align), "::", stringify!(data)),
    );
}
impl flexarray_align<[::std::os::raw::c_int]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(::std::ptr::null(), len);
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    /// Construct a DST for `#canonical_ident` from a thin
    /// pointer.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr<'a>(
        ptr: *const flexarray_align<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> &'a Self {
        let ptr: *const Self = ::std::ptr::from_raw_parts(ptr as *const (), len);
        &*ptr
    }
    /// Construct a mutable DST for `#canonical_ident` from
    /// a thin pointer. This is `MaybeUninit` to allow for
    /// initialization.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    /// Note: returned lifetime is unbounded.
    pub unsafe fn from_ptr_mut<'a>(
        ptr: *mut flexarray_align<[::std::os::raw::c_int; 0]>,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'a mut Self> {
        let ptr: *mut Self = ::std::ptr::from_raw_parts_mut(ptr as *mut (), len);
        ::std::mem::MaybeUninit::new(&mut *ptr)
    }
}
impl flexarray_align<[::std::os::raw::c_int; 0]> {
    /// Turn a sized reference for `#canonical_ident` into
    /// DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref(
        &self,
        len: usize,
    ) -> &flexarray_align<[::std::os::raw::c_int]> {
        unsafe { flexarray_align::<[::std::os::raw::c_int]>::from_ptr(self, len) }
    }
    /// Turn a mutable sized reference for
    /// `#canonical_ident` into DST with the given `len`.
    ///
    /// SAFETY: the `len` must be <= the underlying storage.
    pub unsafe fn from_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_align<[::std::os::raw::c_int]> {
        unsafe {
            flexarray_align::<[::std::os::raw::c_int]>::from_ptr_mut(self, len)
                .assume_init()
        }
    }
}
impl Default for flexarray_align<[::std::os::raw::c_int; 0]> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
