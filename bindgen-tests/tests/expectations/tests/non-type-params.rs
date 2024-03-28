#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Array16 = u8;
pub type ArrayInt4 = [u32; 4usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UsesArray {
    pub array_char_16: [u8; 16usize],
    pub array_bool_8: [u8; 8usize],
    pub array_int_4: ArrayInt4,
}
const _: () = {
    assert!(::std::mem::size_of::<UsesArray>() == 40usize, "Size of UsesArray");
    assert!(::std::mem::align_of::<UsesArray>() == 4usize, "Alignment of UsesArray");
    assert!(
        ::std::mem::offset_of!(UsesArray, array_char_16) == 0usize,
        "Offset of field: UsesArray::array_char_16",
    );
    assert!(
        ::std::mem::offset_of!(UsesArray, array_bool_8) == 16usize,
        "Offset of field: UsesArray::array_bool_8",
    );
    assert!(
        ::std::mem::offset_of!(UsesArray, array_int_4) == 24usize,
        "Offset of field: UsesArray::array_int_4",
    );
};
