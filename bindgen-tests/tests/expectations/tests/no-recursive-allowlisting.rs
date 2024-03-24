#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub enum Bar {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub baz: *mut Bar,
}
const _: () = {
    assert!(::std::mem::size_of::<Foo>() == 8usize, "Size of Foo");
    assert!(::std::mem::align_of::<Foo>() == 8usize, "Alignment of Foo");
    assert!(::std::mem::offset_of!(Foo, baz) == 0usize, "Offset of field: Foo::baz");
};
impl Default for Foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
