#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Foo_empty {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo_empty() {
    assert_eq!(
        ::std::mem::size_of::<Foo_empty>(),
        1usize,
        concat!("Size of: ", stringify!(Foo_empty))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo_empty>(),
        1usize,
        concat!("Alignment of ", stringify!(Foo_empty))
    );
}
impl Clone for Foo_empty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Foo {
    _unused: [u8; 0],
}
impl Clone for Foo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Bar {
    pub f: *mut Foo,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        8usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Bar>())).f as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(f))
    );
}
impl Clone for Bar {
    fn clone(&self) -> Self {
        *self
    }
}
impl Default for Bar {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z10baz_structP3Foo"]
    pub fn baz_struct(f: *mut Foo);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Union {
    _unused: [u8; 0],
}
impl Clone for Union {
    fn clone(&self) -> Self {
        *self
    }
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_unionP5Union"]
    pub fn baz_union(u: *mut Union);
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Quux {
    _unused: [u8; 0],
}
impl Clone for Quux {
    fn clone(&self) -> Self {
        *self
    }
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_classP4Quux"]
    pub fn baz_class(q: *mut Quux);
}
