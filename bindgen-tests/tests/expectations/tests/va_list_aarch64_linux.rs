#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
pub type va_list = __BindgenOpaqueArray<u64, 4usize>;
unsafe extern "C" {
    pub fn vprintf(
        format: *const ::std::os::raw::c_char,
        vlist: __BindgenOpaqueArray<u64, 4usize>,
    ) -> ::std::os::raw::c_int;
}
