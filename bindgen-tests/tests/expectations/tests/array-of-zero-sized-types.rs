#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// This should get an `_address` byte.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Empty {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Empty"][::std::mem::size_of::<Empty>() - 1usize];
    ["Alignment of Empty"][::std::mem::align_of::<Empty>() - 1usize];
};
/// This should not get an `_address` byte, since each `Empty` gets one, meaning
/// that this object is addressable.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasArrayOfEmpty {
    pub empties: [Empty; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HasArrayOfEmpty"][::std::mem::size_of::<HasArrayOfEmpty>() - 10usize];
    ["Alignment of HasArrayOfEmpty"][::std::mem::align_of::<HasArrayOfEmpty>() - 1usize];
    [
        "Offset of field: HasArrayOfEmpty::empties",
    ][::std::mem::offset_of!(HasArrayOfEmpty, empties) - 0usize];
};
