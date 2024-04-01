#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub m_member: ::std::os::raw::c_int,
    pub m_other: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of C"][::std::mem::size_of::<C>() - 8usize];
    ["Alignment of C"][::std::mem::align_of::<C>() - 4usize];
    ["Offset of field: C::m_member"][::std::mem::offset_of!(C, m_member) - 0usize];
    ["Offset of field: C::m_other"][::std::mem::offset_of!(C, m_other) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiable {
    pub m_member: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of NonCopiable"][::std::mem::size_of::<NonCopiable>() - 4usize];
    ["Alignment of NonCopiable"][::std::mem::align_of::<NonCopiable>() - 4usize];
    [
        "Offset of field: NonCopiable::m_member",
    ][::std::mem::offset_of!(NonCopiable, m_member) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiableWithNonCopiableMutableMember {
    pub m_member: NonCopiable,
}
const _: () = {
    [
        "Size of NonCopiableWithNonCopiableMutableMember",
    ][::std::mem::size_of::<NonCopiableWithNonCopiableMutableMember>() - 4usize];
    [
        "Alignment of NonCopiableWithNonCopiableMutableMember",
    ][::std::mem::align_of::<NonCopiableWithNonCopiableMutableMember>() - 4usize];
    [
        "Offset of field: NonCopiableWithNonCopiableMutableMember::m_member",
    ][::std::mem::offset_of!(NonCopiableWithNonCopiableMutableMember, m_member)
        - 0usize];
};
