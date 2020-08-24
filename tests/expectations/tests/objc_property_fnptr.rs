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
    unsafe fn func(
        &self,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_short,
            arg3: f32,
        ) -> ::std::os::raw::c_int,
    >
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, func)
    }
    unsafe fn setFunc_(
        &self,
        func: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: ::std::os::raw::c_char,
                arg2: ::std::os::raw::c_short,
                arg3: f32,
            ) -> ::std::os::raw::c_int,
        >,
    ) where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(*self, setFunc: func)
    }
}
