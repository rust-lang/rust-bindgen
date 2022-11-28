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
    pub fn new() -> Self {
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
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NastyStruct {
    pub mIsSome: bool,
    pub mStorage: NastyStruct__bindgen_ty_1,
    pub __bindgen_anon_1: NastyStruct__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NastyStruct__bindgen_ty_1 {
    pub mFoo: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mDummy: __BindgenUnionField<::std::os::raw::c_ulong>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct NastyStruct__bindgen_ty_2 {
    pub wat: __BindgenUnionField<::std::os::raw::c_short>,
    pub wut: __BindgenUnionField<*mut ::std::os::raw::c_int>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Whatever {
    pub mTPtr: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mInt: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u64,
}
