#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub type_: ::std::os::raw::c_int,
    pub type__: ::std::os::raw::c_long,
    pub type___: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_foo() {
    const UNINIT: ::std::mem::MaybeUninit<foo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        24usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type__) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type__)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).type___) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(type___)
        )
    );
}
pub const Type_let_: Type = 0;
pub const Type_match_: Type = 1;
pub const Type_match__: Type = 2;
pub type Type = ::std::os::raw::c_uint;
extern "C" {
    #[link_name = "\u{1}type"]
    pub fn type__(type___: Type, type__: Type) -> ::std::os::raw::c_int;
}
