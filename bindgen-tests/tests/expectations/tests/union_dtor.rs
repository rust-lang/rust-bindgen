#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub union UnionWithDtor {
    pub mFoo: ::std::os::raw::c_int,
    pub mBar: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UnionWithDtor"][::std::mem::size_of::<UnionWithDtor>() - 8usize];
    ["Alignment of UnionWithDtor"][::std::mem::align_of::<UnionWithDtor>() - 8usize];
    [
        "Offset of field: UnionWithDtor::mFoo",
    ][::std::mem::offset_of!(UnionWithDtor, mFoo) - 0usize];
    [
        "Offset of field: UnionWithDtor::mBar",
    ][::std::mem::offset_of!(UnionWithDtor, mBar) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN13UnionWithDtorD1Ev"]
    pub fn UnionWithDtor_UnionWithDtor_destructor(this: *mut UnionWithDtor);
}
impl Default for UnionWithDtor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl UnionWithDtor {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        UnionWithDtor_UnionWithDtor_destructor(self)
    }
}
