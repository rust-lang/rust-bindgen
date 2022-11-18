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
pub struct basic_string {
    pub _address: u8,
}
pub type basic_string_size_type = ::std::os::raw::c_ulonglong;
pub type basic_string_value_type = ::std::os::raw::c_char;
pub type basic_string_pointer = *mut basic_string_value_type;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___long {
    pub __cap_: basic_string_size_type,
    pub __size_: basic_string_size_type,
    pub __data_: basic_string_pointer,
}
impl Default for basic_string___long {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
pub const basic_string___min_cap: basic_string__bindgen_ty_1 =
    basic_string__bindgen_ty_1::__min_cap;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum basic_string__bindgen_ty_1 {
    __min_cap = 0,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___short {
    pub __bindgen_anon_1: basic_string___short__bindgen_ty_1,
    pub __data_: *mut basic_string_value_type,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___short__bindgen_ty_1 {
    pub __size_: __BindgenUnionField<::std::os::raw::c_uchar>,
    pub __lx: __BindgenUnionField<basic_string_value_type>,
    pub bindgen_union_field: u8,
}
impl Default for basic_string___short {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___ulx {
    pub __lx: __BindgenUnionField<basic_string___long>,
    pub __lxx: __BindgenUnionField<basic_string___short>,
    pub bindgen_union_field: [u8; 0usize],
}
pub const basic_string___n_words: basic_string__bindgen_ty_2 =
    basic_string__bindgen_ty_2::__n_words;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum basic_string__bindgen_ty_2 {
    __n_words = 0,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___raw {
    pub __words: *mut basic_string_size_type,
}
impl Default for basic_string___raw {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___rep {
    pub __bindgen_anon_1: basic_string___rep__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct basic_string___rep__bindgen_ty_1 {
    pub __l: __BindgenUnionField<basic_string___long>,
    pub __s: __BindgenUnionField<basic_string___short>,
    pub __r: __BindgenUnionField<basic_string___raw>,
    pub bindgen_union_field: [u8; 0usize],
}
