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
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
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
        Self(unsafe { msg_send!(class!(Bar), alloc) })
    }
}
impl IFoo for Bar {}
impl From<Bar> for Foo {
    fn from(child: Bar) -> Foo {
        Foo(child.0)
    }
}
impl std::convert::TryFrom<Foo> for Bar {
    type Error = &'static str;
    fn try_from(parent: Foo) -> Result<Bar, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(parent, isKindOfClass : class!(Bar)) };
        if is_kind_of {
            Ok(Bar(parent.0))
        } else {
            Err("This Foo cannot be downcasted to Bar")
        }
    }
}
impl IBar for Bar {}
pub trait IBar: Sized + std::ops::Deref {}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
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
        Self(unsafe { msg_send!(class!(Baz), alloc) })
    }
}
impl IBar for Baz {}
impl From<Baz> for Bar {
    fn from(child: Baz) -> Bar {
        Bar(child.0)
    }
}
impl std::convert::TryFrom<Bar> for Baz {
    type Error = &'static str;
    fn try_from(parent: Bar) -> Result<Baz, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(parent, isKindOfClass : class!(Baz)) };
        if is_kind_of {
            Ok(Baz(parent.0))
        } else {
            Err("This Bar cannot be downcasted to Baz")
        }
    }
}
impl IFoo for Baz {}
impl From<Baz> for Foo {
    fn from(child: Baz) -> Foo {
        Foo(child.0)
    }
}
impl std::convert::TryFrom<Foo> for Baz {
    type Error = &'static str;
    fn try_from(parent: Foo) -> Result<Baz, Self::Error> {
        let is_kind_of: bool = unsafe { msg_send!(parent, isKindOfClass : class!(Baz)) };
        if is_kind_of {
            Ok(Baz(parent.0))
        } else {
            Err("This Foo cannot be downcasted to Baz")
        }
    }
}
impl IBaz for Baz {}
pub trait IBaz: Sized + std::ops::Deref {}
