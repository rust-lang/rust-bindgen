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
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub arr: __BindgenOpaqueArray<u32, 3usize>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 12usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::arr"][::std::mem::offset_of!(C, arr) - 0usize];
};
