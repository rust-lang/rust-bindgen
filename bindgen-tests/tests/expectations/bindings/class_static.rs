#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MyClass {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN7MyClass7exampleE"]
    pub static mut MyClass_example: *const ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_ZN7MyClass26example_check_no_collisionE"]
    pub static mut MyClass_example_check_no_collision:
        *const ::std::os::raw::c_int;
}
#[test]
fn bindgen_test_layout_MyClass() {
    assert_eq!(
        ::std::mem::size_of::<MyClass>(),
        1usize,
        concat!("Size of: ", stringify!(MyClass))
    );
    assert_eq!(
        ::std::mem::align_of::<MyClass>(),
        1usize,
        concat!("Alignment of ", stringify!(MyClass))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZL26example_check_no_collision"]
    pub static mut example_check_no_collision: *const ::std::os::raw::c_int;
}
