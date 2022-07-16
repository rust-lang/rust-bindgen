#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_whatever() {
    assert_eq!(
        ::std::mem::size_of::<whatever>(),
        1usize,
        concat!("Size of: ", stringify!(whatever))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever>(),
        1usize,
        concat!("Alignment of ", stringify!(whatever))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_whatever_child() {
    assert_eq!(
        ::std::mem::size_of::<whatever_child>(),
        1usize,
        concat!("Size of: ", stringify!(whatever_child))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever_child>(),
        1usize,
        concat!("Alignment of ", stringify!(whatever_child))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct whatever_child_with_member {
    pub m_member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_whatever_child_with_member() {
    const UNINIT: ::std::mem::MaybeUninit<whatever_child_with_member> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<whatever_child_with_member>(),
        4usize,
        concat!("Size of: ", stringify!(whatever_child_with_member))
    );
    assert_eq!(
        ::std::mem::align_of::<whatever_child_with_member>(),
        4usize,
        concat!("Alignment of ", stringify!(whatever_child_with_member))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).m_member) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(whatever_child_with_member),
            "::",
            stringify!(m_member)
        )
    );
}
