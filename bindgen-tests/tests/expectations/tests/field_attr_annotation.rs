#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// <div rustbindgen deriveDebug></div>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    /// <div rustbindgen attribute="cfg(test)"></div>
    #[cfg(test)]
    pub x: ::std::os::raw::c_int,
    /// <div rustbindgen attribute="allow(dead_code)"></div>
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
/// <div rustbindgen deriveDebug></div>
#[repr(C)]
#[derive(Clone, Copy)]
pub union Data {
    /// <div rustbindgen attribute="allow(dead_code)"></div>
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
/// <div rustbindgen attribute="cfg(test)"></div>
pub type Handle = ::std::os::raw::c_int;
