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
pub trait IFoo: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Clone)]
pub struct Bar(pub id);
impl std::ops::Deref for Bar {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Bar {}
impl Bar {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(Bar), alloc) })
    }
}
impl IFoo for Bar {}
impl From<Bar> for Foo {
    fn from(child: Bar) -> Foo {
        Foo(child.0)
    }
}
impl IBar for Bar {}
pub trait IBar: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Clone)]
pub struct Baz(pub id);
impl std::ops::Deref for Baz {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Baz {}
impl Baz {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(Baz), alloc) })
    }
}
impl IBar for Baz {}
impl From<Baz> for Bar {
    fn from(child: Baz) -> Bar {
        Bar(child.0)
    }
}
impl IFoo for Baz {}
impl From<Baz> for Foo {
    fn from(child: Baz) -> Foo {
        Foo(child.0)
    }
}
impl IBaz for Baz {}
pub trait IBaz: Sized + std::ops::Deref {}
