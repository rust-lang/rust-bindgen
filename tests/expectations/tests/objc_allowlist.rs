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
pub trait PSomeProtocol: Sized + std::ops::Deref {
    unsafe fn protocolMethod(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, protocolMethod)
    }
    unsafe fn protocolClassMethod()
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(SomeProtocol), protocolClassMethod)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct AllowlistMe(pub id);
impl std::ops::Deref for AllowlistMe {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for AllowlistMe {}
impl AllowlistMe {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(AllowlistMe), alloc) })
    }
}
impl PSomeProtocol for AllowlistMe {}
impl IAllowlistMe for AllowlistMe {}
pub trait IAllowlistMe: Sized + std::ops::Deref {
    unsafe fn method(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, method)
    }
    unsafe fn classMethod()
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(AllowlistMe), classMethod)
    }
}
impl AllowlistMe_InterestingCategory for AllowlistMe {}
pub trait AllowlistMe_InterestingCategory: Sized + std::ops::Deref {}
