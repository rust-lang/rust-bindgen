#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo_empty {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo_empty>() == 1usize, "Size of Foo_empty");
    assert!(::std::mem::align_of::<Foo_empty>() == 1usize, "Alignment of Foo_empty");
};
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
const _: () = {
    assert!(::std::mem::size_of::<Bar>() == 8usize, "Size of Bar");
    assert!(::std::mem::align_of::<Bar>() == 8usize, "Alignment of Bar");
    assert!(::std::mem::offset_of!(Bar, f) == 0usize, "Offset of field: Bar::f");
};
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
