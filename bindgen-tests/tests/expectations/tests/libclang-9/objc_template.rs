#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]
use objc::{self, msg_send, sel, sel_impl, class};
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
impl<ObjectType: 'static> IFoo<ObjectType> for Foo {}
pub trait IFoo<ObjectType: 'static>: Sized + std::ops::Deref {
    unsafe fn get(&self) -> u64
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(* self, get)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct FooMultiGeneric(pub id);
impl std::ops::Deref for FooMultiGeneric {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for FooMultiGeneric {}
impl FooMultiGeneric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(FooMultiGeneric), alloc) })
    }
}
impl<KeyType: 'static, ObjectType: 'static> IFooMultiGeneric<KeyType, ObjectType>
for FooMultiGeneric {}
pub trait IFooMultiGeneric<
    KeyType: 'static,
    ObjectType: 'static,
>: Sized + std::ops::Deref {
    unsafe fn objectForKey_(&self, key: u64) -> u64
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(* self, objectForKey : key)
    }
}
