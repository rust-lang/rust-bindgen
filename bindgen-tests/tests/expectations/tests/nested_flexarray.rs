#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(feature = "nightly")]
#![feature(ptr_metadata, layout_for_ptr)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct Field<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub count: ::std::os::raw::c_int,
    pub data: FAM,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Field"][::std::mem::size_of::<Field>() - 4usize];
    ["Alignment of Field"][::std::mem::align_of::<Field>() - 4usize];
    ["Offset of field: Field::count"][::std::mem::offset_of!(Field, count) - 0usize];
    ["Offset of field: Field::data"][::std::mem::offset_of!(Field, data) - 4usize];
};
impl Field<[::std::os::raw::c_int]> {
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
    pub fn fixed(&self) -> (&Field<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const Field<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(&mut self) -> (&mut Field<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut Field<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl Field<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &Field<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut Field<[::std::os::raw::c_int]> {
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
    ) -> &'unbounded Field<[::std::os::raw::c_int]> {
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
    ) -> ::std::mem::MaybeUninit<&'unbounded mut Field<[::std::os::raw::c_int]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut Field<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut Field<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct Name<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub id: ::std::os::raw::c_int,
    pub field: Field<FAM>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Name"][::std::mem::size_of::<Name>() - 8usize];
    ["Alignment of Name"][::std::mem::align_of::<Name>() - 4usize];
    ["Offset of field: Name::id"][::std::mem::offset_of!(Name, id) - 0usize];
    ["Offset of field: Name::field"][::std::mem::offset_of!(Name, field) - 4usize];
};
impl Name<[::std::os::raw::c_int]> {
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
    pub fn fixed(&self) -> (&Name<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const Name<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(&mut self) -> (&mut Name<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut Name<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl Name<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &Name<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut Name<[::std::os::raw::c_int]> {
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
    ) -> &'unbounded Name<[::std::os::raw::c_int]> {
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
    ) -> ::std::mem::MaybeUninit<&'unbounded mut Name<[::std::os::raw::c_int]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut Name<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut Name<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
    }
}
#[repr(C, packed)]
pub struct NamePacked<FAM: ?Sized = [::std::os::raw::c_int; 0]> {
    pub id: ::std::os::raw::c_int,
    pub field: ::std::mem::ManuallyDrop<Field<FAM>>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NamePacked"][::std::mem::size_of::<NamePacked>() - 8usize];
    ["Alignment of NamePacked"][::std::mem::align_of::<NamePacked>() - 1usize];
    ["Offset of field: NamePacked::id"][::std::mem::offset_of!(NamePacked, id) - 0usize];
    [
        "Offset of field: NamePacked::field",
    ][::std::mem::offset_of!(NamePacked, field) - 4usize];
};
impl NamePacked<[::std::os::raw::c_int]> {
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
    pub fn fixed(&self) -> (&NamePacked<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *const Self).to_raw_parts();
            (&*(ptr as *const NamePacked<[::std::os::raw::c_int; 0]>), len)
        }
    }
    #[inline]
    pub fn fixed_mut(&mut self) -> (&mut NamePacked<[::std::os::raw::c_int; 0]>, usize) {
        unsafe {
            let (ptr, len) = (self as *mut Self).to_raw_parts();
            (&mut *(ptr as *mut NamePacked<[::std::os::raw::c_int; 0]>), len)
        }
    }
}
impl NamePacked<[::std::os::raw::c_int; 0]> {
    /// Convert a sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    pub unsafe fn flex_ref(&self, len: usize) -> &NamePacked<[::std::os::raw::c_int]> {
        Self::flex_ptr(self, len)
    }
    /// Convert a mutable sized prefix to an unsized structure with the given length.
    ///
    /// SAFETY: Underlying storage is initialized up to at least `len` elements.
    #[inline]
    pub unsafe fn flex_ref_mut(
        &mut self,
        len: usize,
    ) -> &mut NamePacked<[::std::os::raw::c_int]> {
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
    ) -> &'unbounded NamePacked<[::std::os::raw::c_int]> {
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
    ) -> ::std::mem::MaybeUninit<&'unbounded mut NamePacked<[::std::os::raw::c_int]>> {
        let mut uninit = ::std::mem::MaybeUninit::<
            &mut NamePacked<[::std::os::raw::c_int]>,
        >::uninit();
        (uninit.as_mut_ptr() as *mut *mut NamePacked<[::std::os::raw::c_int]>)
            .write(::std::ptr::from_raw_parts_mut(ptr as *mut (), len));
        uninit
    }
}
impl Default for NamePacked<[::std::os::raw::c_int; 0]> {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
