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
    fn test_field_m_member() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<C>::uninit();
                let ptr = uninit.as_ptr();
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
    }
    test_field_m_member();
    fn test_field_m_other() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<C>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m_other) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(C),
                "::",
                stringify!(m_other)
            )
        );
    }
    test_field_m_other();
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiable {
    pub m_member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NonCopiable() {
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
    fn test_field_m_member() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<NonCopiable>::uninit();
                let ptr = uninit.as_ptr();
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
    test_field_m_member();
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct NonCopiableWithNonCopiableMutableMember {
    pub m_member: NonCopiable,
}
#[test]
fn bindgen_test_layout_NonCopiableWithNonCopiableMutableMember() {
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
    fn test_field_m_member() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    NonCopiableWithNonCopiableMutableMember,
                >::uninit();
                let ptr = uninit.as_ptr();
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
    test_field_m_member();
}
