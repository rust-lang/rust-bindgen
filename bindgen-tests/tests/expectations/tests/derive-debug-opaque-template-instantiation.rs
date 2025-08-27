#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Instance {
    pub val: __BindgenOpaqueArray<[u32; 50usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Instance"][::std::mem::size_of::<Instance>() - 200usize];
    ["Alignment of Instance"][::std::mem::align_of::<Instance>() - 4usize];
    ["Offset of field: Instance::val"][::std::mem::offset_of!(Instance, val) - 0usize];
};
