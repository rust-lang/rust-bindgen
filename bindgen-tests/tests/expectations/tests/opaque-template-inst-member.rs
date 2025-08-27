#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T: Copy, const N: usize>(pub [T; N]);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<T, N> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct OpaqueTemplate {
    pub _address: u8,
}
/** This should not end up deriving Debug/Hash because its `mBlah` field cannot derive
 Debug/Hash because the instantiation's definition cannot derive Debug/Hash.*/
#[repr(C)]
pub struct ContainsOpaqueTemplate {
    pub mBlah: __BindgenOpaqueArray<u32, 101usize>,
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
impl Default for ContainsOpaqueTemplate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl ::std::cmp::PartialEq for ContainsOpaqueTemplate {
    fn eq(&self, other: &ContainsOpaqueTemplate) -> bool {
        self.mBlah == other.mBlah && self.mBaz == other.mBaz
    }
}
/** This should not end up deriving Debug/Hash either, for similar reasons, although
 we're exercising base member edges now.*/
#[repr(C)]
pub struct InheritsOpaqueTemplate {
    pub _base: __BindgenOpaqueArray<u8, 401usize>,
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
impl ::std::cmp::PartialEq for InheritsOpaqueTemplate {
    fn eq(&self, other: &InheritsOpaqueTemplate) -> bool {
        self._base == other._base && self.wow == other.wow
    }
}
