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
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub arr: __BindgenOpaqueArray<u32, 3usize>,
}
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 12usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::arr"][::std::mem::offset_of!(C, arr) - 0usize];
};
