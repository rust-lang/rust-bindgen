#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(not(test))]
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct JNINativeInterface_ {
    pub GetVersion: ::std::option::Option<
        unsafe extern "stdcall" fn(
            env: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub __hack: ::std::os::raw::c_ulonglong,
}
const _: () = {
    [
        "Size of JNINativeInterface_",
    ][::std::mem::size_of::<JNINativeInterface_>() - 16usize];
    [
        "Alignment of JNINativeInterface_",
    ][::std::mem::align_of::<JNINativeInterface_>() - 8usize];
    [
        "Offset of field: JNINativeInterface_::GetVersion",
    ][::std::mem::offset_of!(JNINativeInterface_, GetVersion) - 0usize];
    [
        "Offset of field: JNINativeInterface_::__hack",
    ][::std::mem::offset_of!(JNINativeInterface_, __hack) - 8usize];
};
extern "stdcall" {
    pub fn bar();
}
