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
#[derive(Clone)]
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
        Self(unsafe { msg_send!(objc::class!(Foo), alloc) })
    }
}
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {
    unsafe fn method(&self)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, method)
    }
    unsafe fn methodWithInt_(&self, foo: ::std::os::raw::c_int)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, methodWithInt: foo)
    }
    unsafe fn methodWithFoo_(&self, foo: Foo)
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, methodWithFoo: foo)
    }
    unsafe fn methodReturningInt(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, methodReturningInt)
    }
    unsafe fn methodReturningFoo(&self) -> Foo
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, methodReturningFoo)
    }
    unsafe fn methodWithArg1_andArg2_andArg3_(
        &self,
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    ) where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send ! ( * self , methodWithArg1 : intvalue andArg2 : ptr andArg3 : floatvalue )
    }
    unsafe fn methodWithAndWithoutKeywords_arg2Name__arg4Name_(
        &self,
        arg1: ::std::os::raw::c_int,
        arg2: f32,
        arg3: f32,
        arg4: ::std::os::raw::c_int,
    ) -> instancetype
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send ! ( * self , methodWithAndWithoutKeywords : arg1 arg2Name : arg2 arg3 : arg3 arg4Name : arg4 )
    }
}
pub type instancetype = id;
