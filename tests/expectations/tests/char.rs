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
    fn test_field_ch() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ch) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(ch)
            )
        );
    }
    test_field_ch();
    fn test_field_u() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize
            },
            1usize,
            concat!("Offset of field: ", stringify!(Test), "::", stringify!(u))
        );
    }
    test_field_u();
    fn test_field_d() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
            },
            2usize,
            concat!("Offset of field: ", stringify!(Test), "::", stringify!(d))
        );
    }
    test_field_d();
    fn test_field_cch() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cch) as usize - ptr as usize
            },
            3usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(cch)
            )
        );
    }
    test_field_cch();
    fn test_field_cu() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cu) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(cu)
            )
        );
    }
    test_field_cu();
    fn test_field_cd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cd) as usize - ptr as usize
            },
            5usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(cd)
            )
        );
    }
    test_field_cd();
    fn test_field_Cch() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Cch) as usize - ptr as usize
            },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Cch)
            )
        );
    }
    test_field_Cch();
    fn test_field_Cu() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Cu) as usize - ptr as usize
            },
            7usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Cu)
            )
        );
    }
    test_field_Cu();
    fn test_field_Cd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Cd) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Cd)
            )
        );
    }
    test_field_Cd();
    fn test_field_Ccch() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Ccch) as usize - ptr as usize
            },
            9usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Ccch)
            )
        );
    }
    test_field_Ccch();
    fn test_field_Ccu() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Ccu) as usize - ptr as usize
            },
            10usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Ccu)
            )
        );
    }
    test_field_Ccu();
    fn test_field_Ccd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Test>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).Ccd) as usize - ptr as usize
            },
            11usize,
            concat!(
                "Offset of field: ",
                stringify!(Test),
                "::",
                stringify!(Ccd)
            )
        );
    }
    test_field_Ccd();
}
