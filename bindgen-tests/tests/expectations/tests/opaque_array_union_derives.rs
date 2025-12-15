#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
/// <div rustbindgen derive="PartialEq"></div>
#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct first_struct {
    pub x: ::std::os::raw::c_int,
    pub __bindgen_padding_0: __BindgenOpaqueArray<[u8; 60usize]>,
    pub padding_to_align: [::std::os::raw::c_char; 60usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of first_struct"][::std::mem::size_of::<first_struct>() - 128usize];
    ["Alignment of first_struct"][::std::mem::align_of::<first_struct>() - 64usize];
    [
        "Offset of field: first_struct::x",
    ][::std::mem::offset_of!(first_struct, x) - 0usize];
    [
        "Offset of field: first_struct::padding_to_align",
    ][::std::mem::offset_of!(first_struct, padding_to_align) - 64usize];
};
impl Default for first_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen derive="PartialEq, Eq"></div>
#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct second_struct {
    pub y: ::std::os::raw::c_int,
    pub __bindgen_padding_0: __BindgenOpaqueArray<[u8; 60usize]>,
    pub more_padding: [::std::os::raw::c_char; 60usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of second_struct"][::std::mem::size_of::<second_struct>() - 128usize];
    ["Alignment of second_struct"][::std::mem::align_of::<second_struct>() - 64usize];
    [
        "Offset of field: second_struct::y",
    ][::std::mem::offset_of!(second_struct, y) - 0usize];
    [
        "Offset of field: second_struct::more_padding",
    ][::std::mem::offset_of!(second_struct, more_padding) - 64usize];
};
impl Default for second_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[repr(align(64))]
#[derive(Clone, Copy, Debug)]
pub struct third_struct {
    pub z: ::std::os::raw::c_int,
    pub __bindgen_padding_0: __BindgenOpaqueArray<[u8; 60usize]>,
    pub final_padding: [::std::os::raw::c_char; 60usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of third_struct"][::std::mem::size_of::<third_struct>() - 128usize];
    ["Alignment of third_struct"][::std::mem::align_of::<third_struct>() - 64usize];
    [
        "Offset of field: third_struct::z",
    ][::std::mem::offset_of!(third_struct, z) - 0usize];
    [
        "Offset of field: third_struct::final_padding",
    ][::std::mem::offset_of!(third_struct, final_padding) - 64usize];
};
impl Default for third_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
