#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Opaque {
    pub _bindgen_opaque_blob: __BindgenOpaqueArray<[u32; 41usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Opaque"][::std::mem::size_of::<Opaque>() - 164usize];
    ["Alignment of Opaque"][::std::mem::align_of::<Opaque>() - 4usize];
};
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OpaqueUser {
    pub opaque: Opaque,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of OpaqueUser"][::std::mem::size_of::<OpaqueUser>() - 164usize];
    ["Alignment of OpaqueUser"][::std::mem::align_of::<OpaqueUser>() - 4usize];
    [
        "Offset of field: OpaqueUser::opaque",
    ][::std::mem::offset_of!(OpaqueUser, opaque) - 0usize];
};
