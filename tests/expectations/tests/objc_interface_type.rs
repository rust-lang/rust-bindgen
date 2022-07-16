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
pub trait IFoo: Sized + std::ops::Deref {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FooStruct {
    pub foo: Foo,
}
#[test]
fn bindgen_test_layout_FooStruct() {
    const UNINIT: ::std::mem::MaybeUninit<FooStruct> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<FooStruct>(),
        8usize,
        concat!("Size of: ", stringify!(FooStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<FooStruct>(),
        8usize,
        concat!("Alignment of ", stringify!(FooStruct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).foo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FooStruct),
            "::",
            stringify!(foo)
        )
    );
}
impl Default for FooStruct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn fooFunc(foo: Foo);
}
extern "C" {
    pub static mut kFoo: Foo;
}
