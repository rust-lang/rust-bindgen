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
/// This should manually derive PartialEq.
#[repr(C)]
#[derive(Copy)]
pub struct ShouldDerivePartialEq {
    pub a: __BindgenUnionField<[::std::os::raw::c_char; 150usize]>,
    pub b: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: [u32; 38usize],
}
#[test]
fn bindgen_test_layout_ShouldDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDerivePartialEq>(),
        152usize,
        concat!("Size of: ", stringify!(ShouldDerivePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDerivePartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldDerivePartialEq))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDerivePartialEq>())).a as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDerivePartialEq),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<ShouldDerivePartialEq>())).b as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ShouldDerivePartialEq),
            "::",
            stringify!(b)
        )
    );
}
impl Clone for ShouldDerivePartialEq {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for ShouldDerivePartialEq {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
impl ::std::cmp::PartialEq for ShouldDerivePartialEq {
    fn eq(&self, other: &ShouldDerivePartialEq) -> bool {
        &self.bindgen_union_field[..] == &other.bindgen_union_field[..]
    }
}
