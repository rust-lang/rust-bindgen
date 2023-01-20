#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Clone, Default)]
pub struct foo_struct {
    pub inner: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo_struct() {
    const UNINIT: ::std::mem::MaybeUninit<foo_struct> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo_struct>(),
        4usize,
        concat!("Size of: ", stringify!(foo_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<foo_struct>(),
        4usize,
        concat!("Alignment of ", stringify!(foo_struct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo_struct),
            "::",
            stringify!(inner)
        )
    );
}
#[repr(u32)]
#[derive(Clone, Hash, PartialEq, Eq, Copy)]
pub enum foo_enum {
    inner = 0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union foo_union {
    pub fst: ::std::mem::ManuallyDrop<::std::os::raw::c_int>,
    pub snd: ::std::mem::ManuallyDrop<f32>,
}
#[test]
fn bindgen_test_layout_foo_union() {
    const UNINIT: ::std::mem::MaybeUninit<foo_union> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo_union>(),
        4usize,
        concat!("Size of: ", stringify!(foo_union))
    );
    assert_eq!(
        ::std::mem::align_of::<foo_union>(),
        4usize,
        concat!("Alignment of ", stringify!(foo_union))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fst) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo_union),
            "::",
            stringify!(fst)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).snd) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo_union),
            "::",
            stringify!(snd)
        )
    );
}
#[repr(C)]
pub struct non_matching {
    pub inner: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_non_matching() {
    const UNINIT: ::std::mem::MaybeUninit<non_matching> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<non_matching>(),
        4usize,
        concat!("Size of: ", stringify!(non_matching))
    );
    assert_eq!(
        ::std::mem::align_of::<non_matching>(),
        4usize,
        concat!("Alignment of ", stringify!(non_matching))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(non_matching),
            "::",
            stringify!(inner)
        )
    );
}
