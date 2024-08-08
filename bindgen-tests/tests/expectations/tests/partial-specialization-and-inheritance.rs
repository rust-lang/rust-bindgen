#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Hash, Debug)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T, const N: usize>(pub [T; N]);
/// `Default` is only implemented for zero-sized opaque types, since
/// Bindgen does not know what contents make sense as a default.
impl<T> Default for __BindgenOpaqueArray<T, 0> {
    fn default() -> Self {
        Self([])
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Derived {
    pub b: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Usage {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN5Usage13static_memberE"]
    pub static mut Usage_static_member: __BindgenOpaqueArray<u32, 2usize>;
}
const _: () = {
    ["Size of Usage"][::std::mem::size_of::<Usage>() - 1usize];
    ["Alignment of Usage"][::std::mem::align_of::<Usage>() - 1usize];
};
