#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<whatever>() == 1usize, "Size of whatever");
    assert!(::std::mem::align_of::<whatever>() == 1usize, "Alignment of whatever");
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<whatever_child>() == 1usize, "Size of whatever_child");
    assert!(
        ::std::mem::align_of::<whatever_child>() == 1usize,
        "Alignment of whatever_child",
    );
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child_with_member {
    pub m_member: ::std::os::raw::c_int,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<whatever_child_with_member>() == 4usize,
        "Size of whatever_child_with_member",
    );
    assert!(
        ::std::mem::align_of::<whatever_child_with_member>() == 4usize,
        "Alignment of whatever_child_with_member",
    );
    assert!(
        ::std::mem::offset_of!(whatever_child_with_member, m_member) == 0usize,
        "Offset of field: whatever_child_with_member::m_member",
    );
};
