#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub f: *mut Foo,
}
#[test]
fn bindgen_test_layout_Bar() {
    const UNINIT: ::std::mem::MaybeUninit<Bar> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
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
        unsafe { ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(f))
    );
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_Z10baz_structP3Foo"]
    pub fn baz_struct(f: *mut Foo);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_unionP5Union"]
    pub fn baz_union(u: *mut Union);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quux {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}_Z9baz_classP4Quux"]
    pub fn baz_class(q: *mut Quux);
}
