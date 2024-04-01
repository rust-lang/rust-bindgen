#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Deriving PartialEq for rust unions is not supported.
#[repr(C)]
#[derive(Copy, Clone)]
pub union ShouldNotDerivePartialEq {
    pub a: ::std::os::raw::c_char,
    pub b: ::std::os::raw::c_int,
}
const _: () = {
    [
        "Size of ShouldNotDerivePartialEq",
    ][::std::mem::size_of::<ShouldNotDerivePartialEq>() - 4usize];
    [
        "Alignment of ShouldNotDerivePartialEq",
    ][::std::mem::align_of::<ShouldNotDerivePartialEq>() - 4usize];
    [
        "Offset of field: ShouldNotDerivePartialEq::a",
    ][::std::mem::offset_of!(ShouldNotDerivePartialEq, a) - 0usize];
    [
        "Offset of field: ShouldNotDerivePartialEq::b",
    ][::std::mem::offset_of!(ShouldNotDerivePartialEq, b) - 0usize];
};
impl Default for ShouldNotDerivePartialEq {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
