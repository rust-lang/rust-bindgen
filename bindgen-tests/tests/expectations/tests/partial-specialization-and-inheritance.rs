#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Derived {
    pub b: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Usage {
    pub _address: u8,
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN5Usage13static_memberE"]
    pub static mut Usage_static_member: __BindgenOpaqueArray<[u32; 2usize]>;
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Usage"][::std::mem::size_of::<Usage>() - 1usize];
    ["Alignment of Usage"][::std::mem::align_of::<Usage>() - 1usize];
};
