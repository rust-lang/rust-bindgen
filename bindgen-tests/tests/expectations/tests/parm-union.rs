#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Struct {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Struct() {
    assert_eq!(
        ::std::mem::size_of::<Struct>(),
        1usize,
        concat!("Size of: ", stringify!(Struct))
    );
    assert_eq!(
        ::std::mem::align_of::<Struct>(),
        1usize,
        concat!("Alignment of ", stringify!(Struct))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN6Struct8FunctionER5Union"]
    pub fn Struct_Function(this: *mut Struct, arg1: *mut Union);
}
impl Struct {
    #[inline]
    pub unsafe fn Function(&mut self, arg1: *mut Union) {
        Struct_Function(self, arg1)
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Union {
    _unused: [u8; 0],
}
