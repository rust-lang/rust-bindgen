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
pub trait PFoo: Sized + std::ops::Deref {}
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
impl PFoo for Foo {}
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
impl PFoo for Bar {}
impl From<Bar> for Foo {
    fn from(child: Bar) -> Foo {
        Foo(child.0)
    }
}
impl std::convert::TryFrom<Foo> for Bar {
    type Error = String;
    fn try_from(parent: Foo) -> Result<Bar, Self::Error> {
        let is_kind_of: bool =
            unsafe { msg_send!(parent, isKindOfClass: class!(Bar)) };
        if is_kind_of {
            Ok(Bar(parent.0))
        } else {
            Err(format!(
                "This {} is not an cannot be downcasted to {}",
                "Foo", "Bar"
            ))
        }
    }
}
impl IBar for Bar {}
pub trait IBar: Sized + std::ops::Deref {}
