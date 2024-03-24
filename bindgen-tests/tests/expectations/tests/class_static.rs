#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
    pub static mut MyClass_example_check_no_collision: *const ::std::os::raw::c_int;
}
const _: () = {
    ["Size of MyClass"][::std::mem::size_of::<MyClass>() - 1usize];
    ["Alignment of MyClass"][::std::mem::align_of::<MyClass>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZL26example_check_no_collision"]
    pub static mut example_check_no_collision: *const ::std::os::raw::c_int;
}
