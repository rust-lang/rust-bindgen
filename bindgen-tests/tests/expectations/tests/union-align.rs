#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Bar {
    pub foo: ::std::os::raw::c_uchar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 16usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 16usize];
    ["Offset of field: Bar::foo"][::std::mem::offset_of!(Bar, foo) - 0usize];
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
#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub union Baz {
    pub bar: Bar,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 16usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 16usize];
    ["Offset of field: Baz::bar"][::std::mem::offset_of!(Baz, bar) - 0usize];
};
impl Default for Baz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
