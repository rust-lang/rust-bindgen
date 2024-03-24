#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub _address: u8,
}
const _: () = {
    ["Size of whatever"][::std::mem::size_of::<whatever>() - 1usize];
    ["Alignment of whatever"][::std::mem::align_of::<whatever>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child {
    pub _address: u8,
}
const _: () = {
    ["Size of whatever_child"][::std::mem::size_of::<whatever_child>() - 1usize];
    ["Alignment of whatever_child"][::std::mem::align_of::<whatever_child>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child_with_member {
    pub m_member: ::std::os::raw::c_int,
}
const _: () = {
    [
        "Size of whatever_child_with_member",
    ][::std::mem::size_of::<whatever_child_with_member>() - 4usize];
    [
        "Alignment of whatever_child_with_member",
    ][::std::mem::align_of::<whatever_child_with_member>() - 4usize];
    [
        "Offset of field: whatever_child_with_member::m_member",
    ][::std::mem::offset_of!(whatever_child_with_member, m_member) - 0usize];
};
