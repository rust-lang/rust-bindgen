#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern "C" {
    pub fn foo() -> ::std::os::raw::c_int;
}
pub type number = ::std::os::raw::c_int;
extern "C" {
    pub fn bar(x: number) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: number,
    pub y: number,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Point"][::std::mem::size_of::<Point>() - 8usize];
    ["Alignment of Point"][::std::mem::align_of::<Point>() - 4usize];
    ["Offset of field: Point::x"][::std::mem::offset_of!(Point, x) - 0usize];
    ["Offset of field: Point::y"][::std::mem::offset_of!(Point, y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Angle {
    pub a: number,
    pub b: number,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Angle"][::std::mem::size_of::<Angle>() - 8usize];
    ["Alignment of Angle"][::std::mem::align_of::<Angle>() - 4usize];
    ["Offset of field: Angle::a"][::std::mem::offset_of!(Angle, a) - 0usize];
    ["Offset of field: Angle::b"][::std::mem::offset_of!(Angle, b) - 4usize];
};
extern "C" {
    pub fn baz(point: Point) -> ::std::os::raw::c_int;
}
pub const NUMBER: number = 42;
