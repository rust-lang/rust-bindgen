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
pub type Array16 = u8;
pub type ArrayInt4 = __BindgenOpaqueArray<u32, 4usize>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UsesArray {
    pub array_char_16: __BindgenOpaqueArray<u8, 16usize>,
    pub array_bool_8: __BindgenOpaqueArray<u8, 8usize>,
    pub array_int_4: ArrayInt4,
}
const _: () = {
    ["Size of UsesArray"][::std::mem::size_of::<UsesArray>() - 40usize];
    ["Alignment of UsesArray"][::std::mem::align_of::<UsesArray>() - 4usize];
    [
        "Offset of field: UsesArray::array_char_16",
    ][::std::mem::offset_of!(UsesArray, array_char_16) - 0usize];
    [
        "Offset of field: UsesArray::array_bool_8",
    ][::std::mem::offset_of!(UsesArray, array_bool_8) - 16usize];
    [
        "Offset of field: UsesArray::array_int_4",
    ][::std::mem::offset_of!(UsesArray, array_int_4) - 24usize];
};
