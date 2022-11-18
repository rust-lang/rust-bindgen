#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type Char = ::std::os::raw::c_char;
pub type SChar = ::std::os::raw::c_schar;
pub type UChar = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Test {
    pub ch: ::std::os::raw::c_char,
    pub u: ::std::os::raw::c_uchar,
    pub d: ::std::os::raw::c_schar,
    pub cch: ::std::os::raw::c_char,
    pub cu: ::std::os::raw::c_uchar,
    pub cd: ::std::os::raw::c_schar,
    pub Cch: Char,
    pub Cu: UChar,
    pub Cd: SChar,
    pub Ccch: Char,
    pub Ccu: UChar,
    pub Ccd: SChar,
}
#[test]
fn bindgen_test_layout_Test() {
    const UNINIT: ::std::mem::MaybeUninit<Test> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        12usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        1usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ch) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(ch))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        1usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(u))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        2usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(d))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cch) as usize - ptr as usize },
        3usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cch))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cu) as usize - ptr as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cu))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).cd) as usize - ptr as usize },
        5usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cd))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Cch) as usize - ptr as usize },
        6usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cch))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Cu) as usize - ptr as usize },
        7usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cu))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Cd) as usize - ptr as usize },
        8usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cd))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ccch) as usize - ptr as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ccu) as usize - ptr as usize },
        10usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccu))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Ccd) as usize - ptr as usize },
        11usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccd))
    );
}
