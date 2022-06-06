#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub mMember: [::std::os::raw::c_longlong; 1usize],
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        8usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_mMember() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mMember) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(mMember)
            )
        );
    }
    test_field_mMember();
}
pub type __m128 = [f32; 4usize];
pub type __m128d = [f64; 2usize];
pub type __m128i = [::std::os::raw::c_longlong; 2usize];
extern "C" {
    #[link_name = "\u{1}_Z3fooDv2_xDv2_d"]
    pub fn foo(arg1: __m128i, arg2: __m128d) -> __m128;
}
