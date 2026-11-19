#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
unsafe extern "C" {
    #[link_name = "takes_va_list__extern"]
    pub fn takes_va_list(va: __BindgenOpaqueArray8<[u8; 32usize]>);
}
unsafe extern "C" {
    #[link_name = "takes_int_and_va_list__extern"]
    pub fn takes_int_and_va_list_wrapped(
        i: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
