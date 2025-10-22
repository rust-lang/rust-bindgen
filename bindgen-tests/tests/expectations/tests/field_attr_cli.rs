#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    #[cfg(test)]
    pub x: ::std::os::raw::c_int,
    #[allow(dead_code)]
    pub y: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Point"][::std::mem::size_of::<Point>() - 8usize];
    ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
    ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
    ["Offset of field: Point::y"][::std::mem::offset_of!(Point, y) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union Data {
    #[allow(dead_code)]
    pub i: ::std::os::raw::c_int,
    pub f: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Data"][::std::mem::size_of::<Data>() - 4usize];
    ["Alignment of Data"][::std::mem::align_of::<Data>() - 4usize];
    ["Offset of field: Data::i"][::std::mem::offset_of!(Data, i) - 0usize];
    ["Offset of field: Data::f"][::std::mem::offset_of!(Data, f) - 0usize];
};
impl Default for Data {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Handle(#[cfg(test)] pub ::std::os::raw::c_int);
