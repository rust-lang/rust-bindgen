#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LittleArray"][::std::mem::size_of::<LittleArray>() - 128usize];
    ["Alignment of LittleArray"][::std::mem::align_of::<LittleArray>() - 4usize];
    ["Offset of field: LittleArray::a"][::std::mem::offset_of!(LittleArray, a) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BigArray"][::std::mem::size_of::<BigArray>() - 132usize];
    ["Alignment of BigArray"][::std::mem::align_of::<BigArray>() - 4usize];
    ["Offset of field: BigArray::a"][::std::mem::offset_of!(BigArray, a) - 0usize];
};
impl Default for BigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WithLittleArray {
    pub a: LittleArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithLittleArray"][::std::mem::size_of::<WithLittleArray>() - 128usize];
    ["Alignment of WithLittleArray"][::std::mem::align_of::<WithLittleArray>() - 4usize];
    [
        "Offset of field: WithLittleArray::a",
    ][::std::mem::offset_of!(WithLittleArray, a) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WithBigArray {
    pub a: BigArray,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of WithBigArray"][::std::mem::size_of::<WithBigArray>() - 132usize];
    ["Alignment of WithBigArray"][::std::mem::align_of::<WithBigArray>() - 4usize];
    [
        "Offset of field: WithBigArray::a",
    ][::std::mem::offset_of!(WithBigArray, a) - 0usize];
};
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
