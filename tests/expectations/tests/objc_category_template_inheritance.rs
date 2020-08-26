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
#[derive(Clone, Copy)]
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
impl<ObjectType: 'static> IFoo<ObjectType> for Foo {}
pub trait IFoo<ObjectType>: Sized + std::ops::Deref {}
impl<ObjectType: 'static> Foo_Baz<ObjectType> for Foo {}
pub trait Foo_Baz<ObjectType>: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Clone, Copy)]
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
impl<ObjectType: 'static> IFoo<ObjectType> for Bar {}
impl<ObjectType: 'static> Foo_Baz<ObjectType> for Bar {}
impl<ObjectType: 'static> IBar<ObjectType> for Bar {}
pub trait IBar<ObjectType>: Sized + std::ops::Deref {}
