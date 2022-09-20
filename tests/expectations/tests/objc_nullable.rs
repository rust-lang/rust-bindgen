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
pub struct Foo(pub id);
impl std::ops::Deref for Foo {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Foo {}
impl Foo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(Foo), alloc) })
    }
}
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {
    unsafe fn nullableReturnType(&self) -> Option<*mut ::std::os::raw::c_int>
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nullableReturnType)
    }
    unsafe fn nonnullableReturnType(&self) -> *mut ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nonnullableReturnType)
    }
    unsafe fn unspecifiedNullabilityReturnType(
        &self,
    ) -> *mut ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, unspecifiedNullabilityReturnType)
    }
    unsafe fn nullableParameter_(&self, foo: Foo)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nullableParameter: foo)
    }
    unsafe fn nonnullableParameter_(&self, foo: Foo)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nonnullableParameter: foo)
    }
    unsafe fn nonnullFoo(&self) -> Foo
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nonnullFoo)
    }
    unsafe fn setNonnullFoo_(&self, nonnullFoo: Foo)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, setNonnullFoo: nonnullFoo)
    }
    unsafe fn nullableFoo(&self) -> Option<Foo>
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, nullableFoo)
    }
    unsafe fn setNullableFoo_(&self, nullableFoo: Foo)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, setNullableFoo: nullableFoo)
    }
}
