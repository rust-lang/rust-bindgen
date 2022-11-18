#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union color {
    pub u1: color__bindgen_ty_1,
    pub u2: color__bindgen_ty_2,
    pub v3: [::std::os::raw::c_uchar; 3usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_color__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<color__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<color__bindgen_ty_1>(),
        3usize,
        concat!("Size of: ", stringify!(color__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<color__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(color__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).g) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_2 {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_color__bindgen_ty_2() {
    const UNINIT: ::std::mem::MaybeUninit<color__bindgen_ty_2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<color__bindgen_ty_2>(),
        3usize,
        concat!("Size of: ", stringify!(color__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<color__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(color__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(v)
        )
    );
}
#[test]
fn bindgen_test_layout_color() {
    const UNINIT: ::std::mem::MaybeUninit<color> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<color>(),
        3usize,
        concat!("Size of: ", stringify!(color))
    );
    assert_eq!(
        ::std::mem::align_of::<color>(),
        1usize,
        concat!("Alignment of ", stringify!(color))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v3) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(color), "::", stringify!(v3))
    );
}
impl Default for color {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
