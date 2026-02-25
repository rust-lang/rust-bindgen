#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct test_long_double_layout {
    pub value: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of test_long_double_layout",
    ][::std::mem::size_of::<test_long_double_layout>() - 16usize];
    [
        "Alignment of test_long_double_layout",
    ][::std::mem::align_of::<test_long_double_layout>() - 16usize];
    [
        "Offset of field: test_long_double_layout::value",
    ][::std::mem::offset_of!(test_long_double_layout, value) - 0usize];
};
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Default, Copy, Clone)]
pub struct test_max_align_sim {
    pub field1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: __BindgenOpaqueArray8<[u8; 8usize]>,
    pub field2: u128,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of test_max_align_sim",
    ][::std::mem::size_of::<test_max_align_sim>() - 32usize];
    [
        "Alignment of test_max_align_sim",
    ][::std::mem::align_of::<test_max_align_sim>() - 16usize];
    [
        "Offset of field: test_max_align_sim::field1",
    ][::std::mem::offset_of!(test_max_align_sim, field1) - 0usize];
    [
        "Offset of field: test_max_align_sim::field2",
    ][::std::mem::offset_of!(test_max_align_sim, field2) - 16usize];
};
