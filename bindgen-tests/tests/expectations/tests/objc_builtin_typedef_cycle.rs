#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
use objc::{self, msg_send, sel, sel_impl, class};
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(C)]
#[derive(Debug)]
pub struct objc_class {
    _unused: [u8; 0],
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Class(pub id);
impl std::ops::Deref for Class {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Class {}
impl Class {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(class!(Class), alloc) })
    }
}
impl IClass for Class {}
pub trait IClass: Sized + std::ops::Deref {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_object {
    pub isa: Class,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of objc_object"][::std::mem::size_of::<objc_object>() - 8usize];
    ["Alignment of objc_object"][::std::mem::align_of::<objc_object>() - 8usize];
    [
        "Offset of field: objc_object::isa",
    ][::std::mem::offset_of!(objc_object, isa) - 0usize];
};
impl Default for objc_object {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub trait PFoo: Sized + std::ops::Deref {
    unsafe fn foo(&self) -> id
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(* self, foo)
    }
}
