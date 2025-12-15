#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Nice {
    pub pointer: Nice_Function,
    pub large_array: [::std::os::raw::c_int; 34usize],
}
pub type Nice_Function = ::std::option::Option<
    unsafe extern "C" fn(data: ::std::os::raw::c_int),
>;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Nice"][::std::mem::size_of::<Nice>() - 144usize];
    ["Alignment of Nice"][::std::mem::align_of::<Nice>() - 8usize];
    ["Offset of field: Nice::pointer"][::std::mem::offset_of!(Nice, pointer) - 0usize];
    [
        "Offset of field: Nice::large_array",
    ][::std::mem::offset_of!(Nice, large_array) - 8usize];
};
impl Default for Nice {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
