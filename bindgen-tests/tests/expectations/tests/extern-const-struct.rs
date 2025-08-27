#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsFoo {
    pub details: [f32; 400usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nsFoo"][::std::mem::size_of::<nsFoo>() - 1600usize];
    ["Alignment of nsFoo"][::std::mem::align_of::<nsFoo>() - 4usize];
    ["Offset of field: nsFoo::details"][::std::mem::offset_of!(nsFoo, details) - 0usize];
};
impl Default for nsFoo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static gDetails: nsFoo;
}
