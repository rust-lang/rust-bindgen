#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SomeClass {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SomeClass"][::std::mem::size_of::<SomeClass>() - 1usize];
    ["Alignment of SomeClass"][::std::mem::align_of::<SomeClass>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN9SomeClassaSERKS_"]
    pub fn SomeClass_operatorequals(
        this: *mut SomeClass,
        another: *const SomeClass,
    ) -> bool;
}
impl SomeClass {
    #[inline]
    pub unsafe fn operatorequals(&mut self, another: *const SomeClass) -> bool {
        SomeClass_operatorequals(self, another)
    }
}
