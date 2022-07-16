#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct struct_a {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_struct_a() {
    const UNINIT: ::std::mem::MaybeUninit<struct_a> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<struct_a>(),
        4usize,
        concat!("Size of: ", stringify!(struct_a))
    );
    assert_eq!(
        ::std::mem::align_of::<struct_a>(),
        4usize,
        concat!("Alignment of ", stringify!(struct_a))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(struct_a),
            "::",
            stringify!(a)
        )
    );
}
pub type a = *const struct_a;
#[repr(C)]
#[derive(Copy, Clone)]
pub union union_b {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_union_b() {
    const UNINIT: ::std::mem::MaybeUninit<union_b> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<union_b>(),
        4usize,
        concat!("Size of: ", stringify!(union_b))
    );
    assert_eq!(
        ::std::mem::align_of::<union_b>(),
        4usize,
        concat!("Alignment of ", stringify!(union_b))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_b),
            "::",
            stringify!(a)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(union_b),
            "::",
            stringify!(b)
        )
    );
}
impl Default for union_b {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type b = union_b;
pub const enum_c_A: enum_c = 0;
pub type enum_c = ::std::os::raw::c_uint;
extern "C" {
    pub fn takes_a(arg: a);
}
extern "C" {
    pub fn takes_b(arg: b);
}
extern "C" {
    pub fn takes_c(arg: enum_c);
}
