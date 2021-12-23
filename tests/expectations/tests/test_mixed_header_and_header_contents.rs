extern "C" {
    pub static mut foo: ::std::option::Option<
        unsafe extern "C" fn(
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub fn bar(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bar2(b: *const ::std::os::raw::c_char) -> f32;
}
pub type Char = ::std::os::raw::c_char;
pub type SChar = ::std::os::raw::c_schar;
pub type UChar = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).ch as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(ch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).u as *const _ as usize },
        1usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(u))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).d as *const _ as usize },
        2usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(d))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).cch as *const _ as usize },
        3usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).cu as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).cd as *const _ as usize },
        5usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Cch as *const _ as usize },
        6usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cch))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Cu as *const _ as usize },
        7usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Cd as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cd))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Ccch as *const _ as usize },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Ccu as *const _ as usize },
        10usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Test>())).Ccd as *const _ as usize },
        11usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccd))
    );
}
