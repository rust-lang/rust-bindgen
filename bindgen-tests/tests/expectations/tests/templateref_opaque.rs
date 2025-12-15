#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct detail_PointerType {
    pub _address: u8,
}
pub type detail_PointerType_Type<T> = *mut T;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct UniquePtr {
    pub _address: u8,
}
pub type UniquePtr_Pointer = detail_PointerType;
