#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct RTCRay {
    pub org: [f32; 3usize],
    pub align0: f32,
    pub dir: [f32; 3usize],
    pub align1: f32,
    pub tnear: f32,
    pub tfar: f32,
    pub time: f32,
    pub mask: ::std::os::raw::c_uint,
    pub Ng: [f32; 3usize],
    pub align2: f32,
    pub u: f32,
    pub v: f32,
    pub geomID: ::std::os::raw::c_uint,
    pub primID: ::std::os::raw::c_uint,
    pub instID: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RTCRay"][::std::mem::size_of::<RTCRay>() - 96usize];
    ["Alignment of RTCRay"][::std::mem::align_of::<RTCRay>() - 16usize];
    ["Offset of field: RTCRay::org"][::std::mem::offset_of!(RTCRay, org) - 0usize];
    [
        "Offset of field: RTCRay::align0",
    ][::std::mem::offset_of!(RTCRay, align0) - 12usize];
    ["Offset of field: RTCRay::dir"][::std::mem::offset_of!(RTCRay, dir) - 16usize];
    [
        "Offset of field: RTCRay::align1",
    ][::std::mem::offset_of!(RTCRay, align1) - 28usize];
    ["Offset of field: RTCRay::tnear"][::std::mem::offset_of!(RTCRay, tnear) - 32usize];
    ["Offset of field: RTCRay::tfar"][::std::mem::offset_of!(RTCRay, tfar) - 36usize];
    ["Offset of field: RTCRay::time"][::std::mem::offset_of!(RTCRay, time) - 40usize];
    ["Offset of field: RTCRay::mask"][::std::mem::offset_of!(RTCRay, mask) - 44usize];
    ["Offset of field: RTCRay::Ng"][::std::mem::offset_of!(RTCRay, Ng) - 48usize];
    [
        "Offset of field: RTCRay::align2",
    ][::std::mem::offset_of!(RTCRay, align2) - 60usize];
    ["Offset of field: RTCRay::u"][::std::mem::offset_of!(RTCRay, u) - 64usize];
    ["Offset of field: RTCRay::v"][::std::mem::offset_of!(RTCRay, v) - 68usize];
    [
        "Offset of field: RTCRay::geomID",
    ][::std::mem::offset_of!(RTCRay, geomID) - 72usize];
    [
        "Offset of field: RTCRay::primID",
    ][::std::mem::offset_of!(RTCRay, primID) - 76usize];
    [
        "Offset of field: RTCRay::instID",
    ][::std::mem::offset_of!(RTCRay, instID) - 80usize];
};
