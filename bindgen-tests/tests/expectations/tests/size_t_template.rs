#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct C {
    pub arr: __BindgenOpaqueArray<[u32; 3usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 12usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::arr"][::std::mem::offset_of!(C, arr) - 0usize];
};
