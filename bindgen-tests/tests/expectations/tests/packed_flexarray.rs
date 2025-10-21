#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#![feature(ptr_metadata, layout_for_ptr)]
#[repr(C, packed)]
pub struct PackedTest<FAM: ?Sized = [::std::os::raw::c_long; 0]> {
    pub Head: ::std::os::raw::c_short,
    pub Tail: ::std::mem::ManuallyDrop<FAM>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PackedTest"][::std::mem::size_of::<PackedTest>() - 2usize];
    ["Alignment of PackedTest"][::std::mem::align_of::<PackedTest>() - 1usize];
    [
        "Offset of field: PackedTest::Head",
    ][::std::mem::offset_of!(PackedTest, Head) - 0usize];
    [
        "Offset of field: PackedTest::Tail",
    ][::std::mem::offset_of!(PackedTest, Tail) - 2usize];
};
impl PackedTest<[::std::os::raw::c_long]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(
                ::std::ptr::null::<()>(),
                len,
            );
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    #[inline]
    pub fn fixed(&self) -> (&PackedTest<[::std::os::raw::c_long; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const PackedTest<[::std::os::raw::c_long; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(
        &mut self,
    ) -> (&mut PackedTest<[::std::os::raw::c_long; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut PackedTest<[::std::os::raw::c_long; 0]>), len)
        }
    }
}
impl PackedTest<[::std::os::raw::c_long; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &PackedTest<[::std::os::raw::c_long]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut PackedTest<[::std::os::raw::c_long]> {
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
    ) -> &'unbounded PackedTest<[::std::os::raw::c_long]> {
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
    ) -> ::std::mem::MaybeUninit<&'unbounded mut PackedTest<[::std::os::raw::c_long]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut PackedTest<[::std::os::raw::c_long]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut PackedTest<[::std::os::raw::c_long]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
    }
}
impl Default for PackedTest<[::std::os::raw::c_long; 0]> {
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
pub struct UnpackedTest<FAM: ?Sized = [::std::os::raw::c_long; 0]> {
    pub Head: ::std::os::raw::c_short,
    pub Tail: FAM,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UnpackedTest"][::std::mem::size_of::<UnpackedTest>() - 8usize];
    ["Alignment of UnpackedTest"][::std::mem::align_of::<UnpackedTest>() - 8usize];
    [
        "Offset of field: UnpackedTest::Head",
    ][::std::mem::offset_of!(UnpackedTest, Head) - 0usize];
    [
        "Offset of field: UnpackedTest::Tail",
    ][::std::mem::offset_of!(UnpackedTest, Tail) - 8usize];
};
impl UnpackedTest<[::std::os::raw::c_long]> {
    pub fn layout(len: usize) -> ::std::alloc::Layout {
        unsafe {
            let p: *const Self = ::std::ptr::from_raw_parts(
                ::std::ptr::null::<()>(),
                len,
            );
            ::std::alloc::Layout::for_value_raw(p)
        }
    }
    #[inline]
    pub fn fixed(&self) -> (&UnpackedTest<[::std::os::raw::c_long; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const UnpackedTest<[::std::os::raw::c_long; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(
        &mut self,
    ) -> (&mut UnpackedTest<[::std::os::raw::c_long; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut UnpackedTest<[::std::os::raw::c_long; 0]>), len)
        }
    }
}
impl UnpackedTest<[::std::os::raw::c_long; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(
        &self,
        len: usize,
    ) -> &UnpackedTest<[::std::os::raw::c_long]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut UnpackedTest<[::std::os::raw::c_long]> {
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
    ) -> &'unbounded UnpackedTest<[::std::os::raw::c_long]> {
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
        &'unbounded mut UnpackedTest<[::std::os::raw::c_long]>,
    > {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut UnpackedTest<[::std::os::raw::c_long]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut UnpackedTest<[::std::os::raw::c_long]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
    }
}
