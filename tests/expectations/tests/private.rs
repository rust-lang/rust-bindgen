#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasPrivate {
    pub mNotPrivate: ::std::os::raw::c_int,
    /// <div rustbindgen private></div>
    mIsPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HasPrivate() {
    assert_eq!(
        ::std::mem::size_of::<HasPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(HasPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<HasPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(HasPrivate))
    );
    fn test_field_mNotPrivate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<HasPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mNotPrivate) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(HasPrivate),
                "::",
                stringify!(mNotPrivate)
            )
        );
    }
    test_field_mNotPrivate();
    fn test_field_mIsPrivate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<HasPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mIsPrivate) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(HasPrivate),
                "::",
                stringify!(mIsPrivate)
            )
        );
    }
    test_field_mIsPrivate();
}
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VeryPrivate {
    mIsPrivate: ::std::os::raw::c_int,
    mIsAlsoPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_VeryPrivate() {
    assert_eq!(
        ::std::mem::size_of::<VeryPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(VeryPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<VeryPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(VeryPrivate))
    );
    fn test_field_mIsPrivate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<VeryPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mIsPrivate) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(VeryPrivate),
                "::",
                stringify!(mIsPrivate)
            )
        );
    }
    test_field_mIsPrivate();
    fn test_field_mIsAlsoPrivate() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<VeryPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mIsAlsoPrivate) as usize -
                    ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(VeryPrivate),
                "::",
                stringify!(mIsAlsoPrivate)
            )
        );
    }
    test_field_mIsAlsoPrivate();
}
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ContradictPrivate {
    /// <div rustbindgen private="false"></div>
    pub mNotPrivate: ::std::os::raw::c_int,
    mIsPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContradictPrivate() {
    assert_eq!(
        ::std::mem::size_of::<ContradictPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(ContradictPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<ContradictPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(ContradictPrivate))
    );
    fn test_field_mNotPrivate() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ContradictPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mNotPrivate) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ContradictPrivate),
                "::",
                stringify!(mNotPrivate)
            )
        );
    }
    test_field_mNotPrivate();
    fn test_field_mIsPrivate() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ContradictPrivate>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mIsPrivate) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(ContradictPrivate),
                "::",
                stringify!(mIsPrivate)
            )
        );
    }
    test_field_mIsPrivate();
}
