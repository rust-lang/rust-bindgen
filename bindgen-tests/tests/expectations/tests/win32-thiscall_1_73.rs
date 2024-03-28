#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target = "i686-pc-windows-msvc")]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>(), 1usize, "Size of Foo");
    assert_eq!(::std::mem::align_of::<Foo>(), 1usize, "Alignment of Foo");
}
extern "thiscall" {
    #[link_name = "\u{1}?test@Foo@@QAEXXZ"]
    pub fn Foo_test(this: *mut Foo);
}
extern "thiscall" {
    #[link_name = "\u{1}?test2@Foo@@QAEHH@Z"]
    pub fn Foo_test2(
        this: *mut Foo,
        var: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
impl Foo {
    #[inline]
    pub unsafe fn test(&mut self) {
        Foo_test(self)
    }
    #[inline]
    pub unsafe fn test2(&mut self, var: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        Foo_test2(self, var)
    }
}
