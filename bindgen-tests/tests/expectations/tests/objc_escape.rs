#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

use objc::{self, class, msg_send, sel, sel_impl};
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct A(pub id);
impl std::ops::Deref for A {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for A {}
impl A {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(A), alloc) })
    }
}
impl IA for A {}
pub trait IA: Sized + std::ops::Deref {
    unsafe fn f_as_(
        &self,
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send ! (* self , f : arg1 r#as : arg2)
    }
}
