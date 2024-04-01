#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rooted {
    pub ptr: MaybeWrapped<::std::os::raw::c_int>,
}
const _: () = {
    ["Size of Rooted"][::std::mem::size_of::<Rooted>() - 4usize];
    ["Alignment of Rooted"][::std::mem::align_of::<Rooted>() - 4usize];
    ["Offset of field: Rooted::ptr"][::std::mem::offset_of!(Rooted, ptr) - 0usize];
};
impl Default for Rooted {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// <div rustbindgen replaces="MaybeWrapped"></div>
pub type MaybeWrapped<a> = a;
const _: () = {
    [
        "Size of template specialization: MaybeWrapped_open0_int_close0",
    ][::std::mem::size_of::<MaybeWrapped<::std::os::raw::c_int>>() - 4usize];
    [
        "Align of template specialization: MaybeWrapped_open0_int_close0",
    ][::std::mem::align_of::<MaybeWrapped<::std::os::raw::c_int>>() - 4usize];
};
