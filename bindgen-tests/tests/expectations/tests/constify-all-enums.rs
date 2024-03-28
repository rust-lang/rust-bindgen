#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub const foo_THIS: foo = 0;
pub const foo_SHOULD_BE: foo = 1;
pub const foo_A_CONSTANT: foo = 2;
pub type foo = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub this_should_work: foo,
}
const _: () = {
    assert!(::std::mem::size_of::<bar>() == 4usize, "Size of bar");
    assert!(::std::mem::align_of::<bar>() == 4usize, "Alignment of bar");
    assert!(
        ::std::mem::offset_of!(bar, this_should_work) == 0usize,
        "Offset of field: bar::this_should_work",
    );
};
impl Default for bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
