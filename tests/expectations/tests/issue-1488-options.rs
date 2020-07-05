#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type OSStatus = ::std::os::raw::c_int;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SomePtr(pub *mut ::std::os::raw::c_void);
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AnotherPtr(pub *mut ::std::os::raw::c_void);
impl ::std::ops::Deref for AnotherPtr {
    type Target = *mut ::std::os::raw::c_void;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::std::ops::DerefMut for AnotherPtr {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
