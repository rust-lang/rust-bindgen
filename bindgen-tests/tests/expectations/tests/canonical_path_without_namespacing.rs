#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
const _: () = {
    assert!(::std::mem::size_of::<Bar>() == 1usize, "Size of Bar");
    assert!(::std::mem::align_of::<Bar>() == 1usize, "Alignment of Bar");
};
extern "C" {
    #[link_name = "\u{1}_Z3bazPN3foo3BarE"]
    pub fn baz(arg1: *mut Bar);
}
