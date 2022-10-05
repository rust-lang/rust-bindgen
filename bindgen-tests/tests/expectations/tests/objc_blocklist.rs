#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct SomeClass(pub id);
impl std::ops::Deref for SomeClass {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for SomeClass {}
impl SomeClass {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(SomeClass), alloc) })
    }
}
impl ISomeClass for SomeClass {}
pub trait ISomeClass: Sized + std::ops::Deref {
    unsafe fn ambiguouslyBlockedMethod(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, ambiguouslyBlockedMethod)
    }
    unsafe fn instanceMethod(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, instanceMethod)
    }
}
