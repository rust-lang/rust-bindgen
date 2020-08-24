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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FooStruct {
    pub foo: Foo,
}
#[test]
fn bindgen_test_layout_FooStruct() {
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
        unsafe {
            &(*(::std::ptr::null::<FooStruct>())).foo as *const _ as usize
        },
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
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    pub fn fooFunc(foo: Foo);
}
extern "C" {
    pub static mut kFoo: Foo;
}
