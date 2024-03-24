#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod foo {
    pub type Type = ::std::os::raw::c_uint;
    pub const Type: Type = 0;
    pub const Type_: Type = 1;
    pub const Type1: Type = 2;
    pub const Type__: Type = 3;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bar {
    pub member: foo::Type,
}
const _: () = {
    assert!(::std::mem::size_of::<bar>() == 4usize, "Size of bar");
    assert!(::std::mem::align_of::<bar>() == 4usize, "Alignment of bar");
    assert!(
        ::std::mem::offset_of!(bar, member) == 0usize,
        "Offset of field: bar::member",
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
