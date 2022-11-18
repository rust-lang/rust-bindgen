#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct C {
    pub m_member: ::std::os::raw::c_int,
    pub m_other: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    const UNINIT: ::std::mem::MaybeUninit<C> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<C>(),
        8usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).m_member) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(m_member)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).m_other) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(C),
            "::",
            stringify!(m_other)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiable {
    pub m_member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NonCopiable() {
    const UNINIT: ::std::mem::MaybeUninit<NonCopiable> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NonCopiable>(),
        4usize,
        concat!("Size of: ", stringify!(NonCopiable))
    );
    assert_eq!(
        ::std::mem::align_of::<NonCopiable>(),
        4usize,
        concat!("Alignment of ", stringify!(NonCopiable))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).m_member) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NonCopiable),
            "::",
            stringify!(m_member)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiableWithNonCopiableMutableMember {
    pub m_member: NonCopiable,
}
#[test]
fn bindgen_test_layout_NonCopiableWithNonCopiableMutableMember() {
    const UNINIT: ::std::mem::MaybeUninit<
        NonCopiableWithNonCopiableMutableMember,
    > = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<NonCopiableWithNonCopiableMutableMember>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(NonCopiableWithNonCopiableMutableMember)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<NonCopiableWithNonCopiableMutableMember>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(NonCopiableWithNonCopiableMutableMember)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).m_member) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NonCopiableWithNonCopiableMutableMember),
            "::",
            stringify!(m_member)
        )
    );
}
