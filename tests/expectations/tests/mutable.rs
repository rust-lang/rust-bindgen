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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<C>() };
            let struct_ptr = &struct_instance as *const C;
            let field_ptr = std::ptr::addr_of!(struct_instance.m_member);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe { std::mem::zeroed::<C>() };
            let struct_ptr = &struct_instance as *const C;
            let field_ptr = std::ptr::addr_of!(struct_instance.m_other);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<NonCopiable>() };
            let struct_ptr = &struct_instance as *const NonCopiable;
            let field_ptr = std::ptr::addr_of!(struct_instance.m_member);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<NonCopiableWithNonCopiableMutableMember>()
            };
            let struct_ptr = &struct_instance
                as *const NonCopiableWithNonCopiableMutableMember;
            let field_ptr = std::ptr::addr_of!(struct_instance.m_member);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
