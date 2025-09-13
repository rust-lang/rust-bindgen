#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Container {
    pub normalPointer: *mut ::std::os::raw::c_int,
    pub constPointer: *const ::std::os::raw::c_int,
    pub normalRef: ::std::ptr::NonNull<::std::os::raw::c_int>,
    pub constRef: ::std::ptr::NonNull<::std::os::raw::c_int>,
    pub pointerRef: ::std::ptr::NonNull<*mut ::std::os::raw::c_int>,
    pub constPointerRef: ::std::ptr::NonNull<*const ::std::os::raw::c_int>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Container"][::std::mem::size_of::<Container>() - 48usize];
    ["Alignment of Container"][::std::mem::align_of::<Container>() - 8usize];
    [
        "Offset of field: Container::normalPointer",
    ][::std::mem::offset_of!(Container, normalPointer) - 0usize];
    [
        "Offset of field: Container::constPointer",
    ][::std::mem::offset_of!(Container, constPointer) - 8usize];
    [
        "Offset of field: Container::normalRef",
    ][::std::mem::offset_of!(Container, normalRef) - 16usize];
    [
        "Offset of field: Container::constRef",
    ][::std::mem::offset_of!(Container, constRef) - 24usize];
    [
        "Offset of field: Container::pointerRef",
    ][::std::mem::offset_of!(Container, pointerRef) - 32usize];
    [
        "Offset of field: Container::constPointerRef",
    ][::std::mem::offset_of!(Container, constPointerRef) - 40usize];
};
impl Default for Container {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z20refReturningFunctionv"]
    pub fn refReturningFunction() -> ::std::ptr::NonNull<::std::os::raw::c_int>;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z20functionConsumingRefRifRKi"]
    pub fn functionConsumingRef(
        someRef: ::std::ptr::NonNull<::std::os::raw::c_int>,
        normalArgument: f32,
        constRef: ::std::ptr::NonNull<::std::os::raw::c_int>,
    );
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z27functionConsumingPointerRefRPi"]
    pub fn functionConsumingPointerRef(
        pointerRef: ::std::ptr::NonNull<*mut ::std::os::raw::c_int>,
    );
}
