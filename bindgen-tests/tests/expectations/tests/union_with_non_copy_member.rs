#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        *self
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of NonCopyType"][::std::mem::size_of::<NonCopyType>() - 4usize];
    ["Alignment of NonCopyType"][::std::mem::align_of::<NonCopyType>() - 4usize];
    [
        "Offset of field: NonCopyType::foo",
    ][::std::mem::offset_of!(NonCopyType, foo) - 0usize];
};
#[repr(C)]
pub struct WithBindgenGeneratedWrapper {
    pub non_copy_type: __BindgenUnionField<NonCopyType>,
    pub bar: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of WithBindgenGeneratedWrapper",
    ][::std::mem::size_of::<WithBindgenGeneratedWrapper>() - 4usize];
    [
        "Alignment of WithBindgenGeneratedWrapper",
    ][::std::mem::align_of::<WithBindgenGeneratedWrapper>() - 4usize];
    [
        "Offset of field: WithBindgenGeneratedWrapper::non_copy_type",
    ][::std::mem::offset_of!(WithBindgenGeneratedWrapper, non_copy_type) - 0usize];
    [
        "Offset of field: WithBindgenGeneratedWrapper::bar",
    ][::std::mem::offset_of!(WithBindgenGeneratedWrapper, bar) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithManuallyDrop"][::std::mem::size_of::<WithManuallyDrop>() - 4usize];
    [
        "Alignment of WithManuallyDrop",
    ][::std::mem::align_of::<WithManuallyDrop>() - 4usize];
    [
        "Offset of field: WithManuallyDrop::non_copy_type",
    ][::std::mem::offset_of!(WithManuallyDrop, non_copy_type) - 0usize];
    [
        "Offset of field: WithManuallyDrop::bar",
    ][::std::mem::offset_of!(WithManuallyDrop, bar) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithDefaultWrapper"][::std::mem::size_of::<WithDefaultWrapper>() - 4usize];
    [
        "Alignment of WithDefaultWrapper",
    ][::std::mem::align_of::<WithDefaultWrapper>() - 4usize];
    [
        "Offset of field: WithDefaultWrapper::non_copy_type",
    ][::std::mem::offset_of!(WithDefaultWrapper, non_copy_type) - 0usize];
    [
        "Offset of field: WithDefaultWrapper::bar",
    ][::std::mem::offset_of!(WithDefaultWrapper, bar) - 0usize];
};
impl Default for WithDefaultWrapper {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
