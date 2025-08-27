#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
pub type va_list = __BindgenOpaqueArray8<[u8; 32usize]>;
unsafe extern "C" {
    pub fn vprintf(
        format: *const ::std::os::raw::c_char,
        vlist: __BindgenOpaqueArray8<[u8; 32usize]>,
    ) -> ::std::os::raw::c_int;
}
