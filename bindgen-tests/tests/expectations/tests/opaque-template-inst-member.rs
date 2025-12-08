#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/** This should not end up deriving Debug/Hash because its `mBlah` field cannot derive
 Debug/Hash because the instantiation's definition cannot derive Debug/Hash.*/
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: __BindgenOpaqueArray<[u32; 101usize]>,
    pub mBaz: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ContainsOpaqueTemplate",
    ][::std::mem::size_of::<ContainsOpaqueTemplate>() - 408usize];
    [
        "Alignment of ContainsOpaqueTemplate",
    ][::std::mem::align_of::<ContainsOpaqueTemplate>() - 4usize];
    [
        "Offset of field: ContainsOpaqueTemplate::mBlah",
    ][::std::mem::offset_of!(ContainsOpaqueTemplate, mBlah) - 0usize];
    [
        "Offset of field: ContainsOpaqueTemplate::mBaz",
    ][::std::mem::offset_of!(ContainsOpaqueTemplate, mBaz) - 404usize];
};
/** This should not end up deriving Debug/Hash either, for similar reasons, although
 we're exercising base member edges now.*/
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InheritsOpaqueTemplate {
    pub _base: __BindgenOpaqueArray<[u8; 401usize]>,
    pub wow: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of InheritsOpaqueTemplate",
    ][::std::mem::size_of::<InheritsOpaqueTemplate>() - 416usize];
    [
        "Alignment of InheritsOpaqueTemplate",
    ][::std::mem::align_of::<InheritsOpaqueTemplate>() - 8usize];
    [
        "Offset of field: InheritsOpaqueTemplate::wow",
    ][::std::mem::offset_of!(InheritsOpaqueTemplate, wow) - 408usize];
};
impl Default for InheritsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
