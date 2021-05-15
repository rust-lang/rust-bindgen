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
    assert_eq!(
        {
            const STRUCT_SIZE: usize = std::mem::size_of::<foo>();
            let buffer = [0u8; STRUCT_SIZE];
            let struct_instance = unsafe {
                std::mem::transmute::<[u8; STRUCT_SIZE], foo>(buffer)
            };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.mMember);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
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
pub type __m128 = [f32; 4usize];
pub type __m128d = [f64; 2usize];
pub type __m128i = [::std::os::raw::c_longlong; 2usize];
extern "C" {
    #[link_name = "\u{1}_Z3fooDv2_xDv2_d"]
    pub fn foo(arg1: __m128i, arg2: __m128d) -> __m128;
}
