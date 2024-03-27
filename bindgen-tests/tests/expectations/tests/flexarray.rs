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
    #[inline]
    pub fn fixed(&self) -> (&flexarray<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const flexarray<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(&mut self) -> (&mut flexarray<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut flexarray<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl flexarray<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &flexarray<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray<[::std::os::raw::c_int]> {
        Self::flex_ptr_mut(self, len).assume_init()
    }
    /// Construct DST variant from a pointer and a size.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage is fully initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr<'unbounded>(
        ptr: *const Self,
        len: usize,
    ) -> &'unbounded flexarray<[::std::os::raw::c_int]> {
        &*::std::ptr::from_raw_parts(ptr as *const (), len)
    }
    /// Construct mutable DST variant from a pointer and a
    /// size. The returned `&mut` reference is initialized
    /// pointing to memory referenced by `ptr`, but there's
    /// no requirement that that memory be initialized.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage has space for at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr_mut<'unbounded>(
        ptr: *mut Self,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'unbounded mut flexarray<[::std::os::raw::c_int]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut flexarray<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut flexarray<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
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
    #[inline]
    pub fn fixed(&self) -> (&flexarray_zero<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const flexarray_zero<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(
        &mut self,
    ) -> (&mut flexarray_zero<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut flexarray_zero<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl flexarray_zero<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(
        &self,
        len: usize,
    ) -> &flexarray_zero<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_zero<[::std::os::raw::c_int]> {
        Self::flex_ptr_mut(self, len).assume_init()
    }
    /// Construct DST variant from a pointer and a size.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage is fully initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr<'unbounded>(
        ptr: *const Self,
        len: usize,
    ) -> &'unbounded flexarray_zero<[::std::os::raw::c_int]> {
        &*::std::ptr::from_raw_parts(ptr as *const (), len)
    }
    /// Construct mutable DST variant from a pointer and a
    /// size. The returned `&mut` reference is initialized
    /// pointing to memory referenced by `ptr`, but there's
    /// no requirement that that memory be initialized.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage has space for at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr_mut<'unbounded>(
        ptr: *mut Self,
        len: usize,
    ) -> ::std::mem::MaybeUninit<
        &'unbounded mut flexarray_zero<[::std::os::raw::c_int]>,
    > {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut flexarray_zero<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut flexarray_zero<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
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
    #[inline]
    pub fn fixed(&self) -> (&flexarray_template<T, [T; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const flexarray_template<T, [T; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(&mut self) -> (&mut flexarray_template<T, [T; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut flexarray_template<T, [T; 0]>), len)
        }
    }
}
impl<T> flexarray_template<T, [T; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &flexarray_template<T, [T]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_template<T, [T]> {
        Self::flex_ptr_mut(self, len).assume_init()
    }
    /// Construct DST variant from a pointer and a size.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage is fully initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr<'unbounded>(
        ptr: *const Self,
        len: usize,
    ) -> &'unbounded flexarray_template<T, [T]> {
        &*::std::ptr::from_raw_parts(ptr as *const (), len)
    }
    /// Construct mutable DST variant from a pointer and a
    /// size. The returned `&mut` reference is initialized
    /// pointing to memory referenced by `ptr`, but there's
    /// no requirement that that memory be initialized.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage has space for at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr_mut<'unbounded>(
        ptr: *mut Self,
        len: usize,
    ) -> ::std::mem::MaybeUninit<&'unbounded mut flexarray_template<T, [T]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut flexarray_template<T, [T]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut flexarray_template<T, [T]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
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
    #[inline]
    pub fn fixed(
        &self,
    ) -> (&flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (
                &*(ptr as *const flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>),
                len,
            )
        }
    }
    #[inline]
    pub fn fixed_mut(
        &mut self,
    ) -> (&mut flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (
                &mut *(ptr
                    as *mut flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]>),
                len,
            )
        }
    }
}
impl flexarray_bogus_zero_fam<[::std::os::raw::c_char; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(
        &self,
        len: usize,
    ) -> &flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
        Self::flex_ptr_mut(self, len).assume_init()
    }
    /// Construct DST variant from a pointer and a size.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage is fully initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr<'unbounded>(
        ptr: *const Self,
        len: usize,
    ) -> &'unbounded flexarray_bogus_zero_fam<[::std::os::raw::c_char]> {
        &*::std::ptr::from_raw_parts(ptr as *const (), len)
    }
    /// Construct mutable DST variant from a pointer and a
    /// size. The returned `&mut` reference is initialized
    /// pointing to memory referenced by `ptr`, but there's
    /// no requirement that that memory be initialized.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage has space for at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr_mut<'unbounded>(
        ptr: *mut Self,
        len: usize,
    ) -> ::std::mem::MaybeUninit<
        &'unbounded mut flexarray_bogus_zero_fam<[::std::os::raw::c_char]>,
    > {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut flexarray_bogus_zero_fam<[::std::os::raw::c_char]>,
        >::uninit();
        (uninit.as_mut_ptr()
            as *mut *mut flexarray_bogus_zero_fam<[::std::os::raw::c_char]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
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
    #[inline]
    pub fn fixed(&self) -> (&flexarray_align<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const flexarray_align<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(
        &mut self,
    ) -> (&mut flexarray_align<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut flexarray_align<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl flexarray_align<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(
        &self,
        len: usize,
    ) -> &flexarray_align<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut flexarray_align<[::std::os::raw::c_int]> {
        Self::flex_ptr_mut(self, len).assume_init()
    }
    /// Construct DST variant from a pointer and a size.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage is fully initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr<'unbounded>(
        ptr: *const Self,
        len: usize,
    ) -> &'unbounded flexarray_align<[::std::os::raw::c_int]> {
        &*::std::ptr::from_raw_parts(ptr as *const (), len)
    }
    /// Construct mutable DST variant from a pointer and a
    /// size. The returned `&mut` reference is initialized
    /// pointing to memory referenced by `ptr`, but there's
    /// no requirement that that memory be initialized.
    ///
    /// NOTE: lifetime of returned reference is not tied to any underlying storage.
    /// SAFETY: `ptr` is valid. Underlying storage has space for at least `len` elements.
    #[inline]
    pub unsafe fn flex_ptr_mut<'unbounded>(
        ptr: *mut Self,
        len: usize,
    ) -> ::std::mem::MaybeUninit<
        &'unbounded mut flexarray_align<[::std::os::raw::c_int]>,
    > {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut flexarray_align<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut flexarray_align<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
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
