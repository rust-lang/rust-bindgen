#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub f: *const Foo,
    pub m: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 16usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::f"][::std::mem::offset_of!(Bar, f) - 0usize];
    ["Offset of field: Bar::m"][::std::mem::offset_of!(Bar, m) - 8usize];
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
#[derive(Debug, Copy, Clone)]
pub struct Baz {
    pub f: *mut Foo,
    pub m: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 16usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 8usize];
    ["Offset of field: Baz::f"][::std::mem::offset_of!(Baz, f) - 0usize];
    ["Offset of field: Baz::m"][::std::mem::offset_of!(Baz, m) - 8usize];
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tar {
    pub f: *const Foo,
    pub m: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Tar"][::std::mem::size_of::<Tar>() - 16usize];
    ["Alignment of Tar"][::std::mem::align_of::<Tar>() - 8usize];
    ["Offset of field: Tar::f"][::std::mem::offset_of!(Tar, f) - 0usize];
    ["Offset of field: Tar::m"][::std::mem::offset_of!(Tar, m) - 8usize];
};
impl Default for Tar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Taz {
    pub f: *mut Foo,
    pub m: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Taz"][::std::mem::size_of::<Taz>() - 16usize];
    ["Alignment of Taz"][::std::mem::align_of::<Taz>() - 8usize];
    ["Offset of field: Taz::f"][::std::mem::offset_of!(Taz, f) - 0usize];
    ["Offset of field: Taz::m"][::std::mem::offset_of!(Taz, m) - 8usize];
};
impl Default for Taz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
