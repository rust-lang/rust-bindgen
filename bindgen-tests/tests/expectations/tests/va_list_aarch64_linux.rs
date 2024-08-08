#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Hash, Debug)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T, const N: usize>(pub [T; N]);
/// `Default` is only implemented for zero-sized opaque types, since
/// Bindgen does not know what contents make sense as a default.
impl<T> Default for __BindgenOpaqueArray<T, 0> {
    fn default() -> Self {
        Self([])
    }
}
pub type va_list = __BindgenOpaqueArray<u64, 4usize>;
extern "C" {
    pub fn vprintf(
        format: *const ::std::os::raw::c_char,
        vlist: __BindgenOpaqueArray<u64, 4usize>,
    ) -> ::std::os::raw::c_int;
}
