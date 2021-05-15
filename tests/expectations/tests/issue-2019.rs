#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        4usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<A>() };
            let struct_ptr = &struct_instance as *const A;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(a))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1A4makeEv"]
    pub fn make() -> A;
}
impl A {
    #[inline]
    pub unsafe fn make() -> A {
        make()
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct B {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(
        ::std::mem::size_of::<B>(),
        4usize,
        concat!("Size of: ", stringify!(B))
    );
    assert_eq!(
        ::std::mem::align_of::<B>(),
        4usize,
        concat!("Alignment of ", stringify!(B))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<B>() };
            let struct_ptr = &struct_instance as *const B;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(B), "::", stringify!(b))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1B4makeEv"]
    pub fn make1() -> B;
}
impl B {
    #[inline]
    pub unsafe fn make() -> B {
        make1()
    }
}
