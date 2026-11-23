#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]
use objc::{self, msg_send, sel, sel_impl, class};
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Base(pub id);
impl std::ops::Deref for Base {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Base {}
impl Base {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(Base), alloc) })
    }
}
impl IBase for Base {}
pub trait IBase: Sized + std::ops::Deref {}
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
        Self(unsafe { msg_send!(class!(RuntimeFoo), alloc) })
    }
}
impl IBase for Foo {}
impl From<Foo> for Base {
    fn from(child: Foo) -> Base {
        Base(child.0)
    }
}
impl std::convert::TryFrom<Base> for Foo {
    type Error = &'static str;
    fn try_from(parent: Base) -> Result<Foo, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(parent, isKindOfClass : class!(RuntimeFoo))
        };
        if is_kind_of {
            Ok(Foo(parent.0))
        } else {
            Err("This Base cannot be downcasted to Foo")
        }
    }
}
impl IFoo for Foo {}
pub trait IFoo: Sized + std::ops::Deref {
    unsafe fn value(&self) -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(* self, value)
    }
    unsafe fn make() -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(RuntimeFoo), make)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Plain(pub id);
impl std::ops::Deref for Plain {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Plain {}
impl Plain {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(Plain), alloc) })
    }
}
impl IBase for Plain {}
impl From<Plain> for Base {
    fn from(child: Plain) -> Base {
        Base(child.0)
    }
}
impl std::convert::TryFrom<Base> for Plain {
    type Error = &'static str;
    fn try_from(parent: Base) -> Result<Plain, Self::Error> {
        let is_kind_of: bool = unsafe {
            msg_send!(parent, isKindOfClass : class!(Plain))
        };
        if is_kind_of {
            Ok(Plain(parent.0))
        } else {
            Err("This Base cannot be downcasted to Plain")
        }
    }
}
impl IPlain for Plain {}
pub trait IPlain: Sized + std::ops::Deref {
    unsafe fn make() -> ::std::os::raw::c_int
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(class!(Plain), make)
    }
}
