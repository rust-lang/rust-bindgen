#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type MustUseInt = ::std::os::raw::c_int;
unsafe extern "C" {
    #[must_use]
    pub fn return_int() -> MustUseInt;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[must_use]
pub struct MustUseStruct {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MustUseStruct"][::std::mem::size_of::<MustUseStruct>() - 4usize];
    ["Alignment of MustUseStruct"][::std::mem::align_of::<MustUseStruct>() - 4usize];
    [
        "Offset of field: MustUseStruct::a",
    ][::std::mem::offset_of!(MustUseStruct, a) - 0usize];
};
unsafe extern "C" {
    #[must_use]
    pub fn return_struct() -> MustUseStruct;
}
/// <div rustbindgen mustusetype></div>
pub type AnnotatedInt = ::std::os::raw::c_int;
unsafe extern "C" {
    #[must_use]
    pub fn return_annotated_int() -> AnnotatedInt;
}
unsafe extern "C" {
    pub fn return_plain_int() -> ::std::os::raw::c_int;
}
/// <div rustbindgen mustusetype></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[must_use]
pub struct AnnotatedStruct {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AnnotatedStruct"][::std::mem::size_of::<AnnotatedStruct>() - 4usize];
    ["Alignment of AnnotatedStruct"][::std::mem::align_of::<AnnotatedStruct>() - 4usize];
    [
        "Offset of field: AnnotatedStruct::a",
    ][::std::mem::offset_of!(AnnotatedStruct, a) - 0usize];
};
unsafe extern "C" {
    #[must_use]
    pub fn return_annotated_struct() -> AnnotatedStruct;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlainStruct {
    pub a: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PlainStruct"][::std::mem::size_of::<PlainStruct>() - 4usize];
    ["Alignment of PlainStruct"][::std::mem::align_of::<PlainStruct>() - 4usize];
    ["Offset of field: PlainStruct::a"][::std::mem::offset_of!(PlainStruct, a) - 0usize];
};
/// <div rustbindgen mustusetype></div>
pub type TypedefPlainStruct = PlainStruct;
unsafe extern "C" {
    pub fn return_plain_struct() -> PlainStruct;
}
unsafe extern "C" {
    #[must_use]
    pub fn return_typedef_struct() -> TypedefPlainStruct;
}
