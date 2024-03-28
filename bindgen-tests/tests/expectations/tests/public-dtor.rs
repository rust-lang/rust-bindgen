#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default)]
pub struct cv_Foo {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<cv_Foo>() == 1usize, "Size of cv_Foo");
    assert!(::std::mem::align_of::<cv_Foo>() == 1usize, "Alignment of cv_Foo");
};
extern "C" {
    #[link_name = "\u{1}_ZN2cv3FooD1Ev"]
    pub fn cv_Foo_Foo_destructor(this: *mut cv_Foo);
}
impl cv_Foo {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        cv_Foo_Foo_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct cv_Bar {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<cv_Bar>() == 1usize, "Size of cv_Bar");
    assert!(::std::mem::align_of::<cv_Bar>() == 1usize, "Alignment of cv_Bar");
};
