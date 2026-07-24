#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct UnparsedTemplate(pub u32);
#[repr(C)]
pub struct OuterStruct {
    pub field: UnparsedTemplate,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OuterStruct"][::std::mem::size_of::<OuterStruct>() - 4usize];
    ["Alignment of OuterStruct"][::std::mem::align_of::<OuterStruct>() - 4usize];
    [
        "Offset of field: OuterStruct::field",
    ][::std::mem::offset_of!(OuterStruct, field) - 0usize];
};
impl Default for OuterStruct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
