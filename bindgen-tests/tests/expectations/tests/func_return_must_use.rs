#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type MustUseInt = ::std::os::raw::c_int;
extern "C" {
    #[must_use]
    pub fn return_int() -> MustUseInt;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct MustUseStruct {
    _unused: [u8; 0],
}
extern "C" {
    #[must_use]
    pub fn return_struct() -> MustUseStruct;
}
/// <div rustbindgen mustusetype></div>
pub type AnnotatedInt = ::std::os::raw::c_int;
extern "C" {
    #[must_use]
    pub fn return_annotated_int() -> AnnotatedInt;
}
extern "C" {
    pub fn return_plain_int() -> ::std::os::raw::c_int;
}
/// <div rustbindgen mustusetype></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
#[must_use]
pub struct AnnotatedStruct {}
const _: () = {
    ["Size of AnnotatedStruct"][::std::mem::size_of::<AnnotatedStruct>() - 0usize];
    ["Alignment of AnnotatedStruct"][::std::mem::align_of::<AnnotatedStruct>() - 1usize];
};
extern "C" {
    #[must_use]
    pub fn return_annotated_struct() -> AnnotatedStruct;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlainStruct {}
const _: () = {
    ["Size of PlainStruct"][::std::mem::size_of::<PlainStruct>() - 0usize];
    ["Alignment of PlainStruct"][::std::mem::align_of::<PlainStruct>() - 1usize];
};
/// <div rustbindgen mustusetype></div>
pub type TypedefPlainStruct = PlainStruct;
extern "C" {
    pub fn return_plain_struct() -> PlainStruct;
}
extern "C" {
    #[must_use]
    pub fn return_typedef_struct() -> TypedefPlainStruct;
}
