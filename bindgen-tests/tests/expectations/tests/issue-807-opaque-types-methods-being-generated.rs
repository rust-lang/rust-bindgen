#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pupper {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Pupper"][::std::mem::size_of::<Pupper>() - 1usize];
    ["Alignment of Pupper"][::std::mem::align_of::<Pupper>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Doggo {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Doggo"][::std::mem::size_of::<Doggo>() - 1usize];
    ["Alignment of Doggo"][::std::mem::align_of::<Doggo>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SuchWow {
    pub _address: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SuchWow"][::std::mem::size_of::<SuchWow>() - 1usize];
    ["Alignment of SuchWow"][::std::mem::align_of::<SuchWow>() - 1usize];
};
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _bindgen_opaque_blob: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Opaque"][::std::mem::size_of::<Opaque>() - 1usize];
    ["Alignment of Opaque"][::std::mem::align_of::<Opaque>() - 1usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN6Opaque17eleven_out_of_tenEv"]
    pub fn Opaque_eleven_out_of_ten(this: *mut Opaque) -> SuchWow;
}
extern "C" {
    #[link_name = "\u{1}_ZN6OpaqueC1E6Pupper"]
    pub fn Opaque_Opaque(this: *mut Opaque, pup: Pupper);
}
impl Opaque {
    #[inline]
    pub unsafe fn eleven_out_of_ten(&mut self) -> SuchWow {
        Opaque_eleven_out_of_ten(self)
    }
    #[inline]
    pub unsafe fn new(pup: Pupper) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Opaque_Opaque(__bindgen_tmp.as_mut_ptr(), pup);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN6Opaque11MAJESTIC_AFE"]
    pub static mut Opaque_MAJESTIC_AF: Doggo;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Allowlisted {
    pub some_member: Opaque,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Allowlisted"][::std::mem::size_of::<Allowlisted>() - 1usize];
    ["Alignment of Allowlisted"][::std::mem::align_of::<Allowlisted>() - 1usize];
    [
        "Offset of field: Allowlisted::some_member",
    ][::std::mem::offset_of!(Allowlisted, some_member) - 0usize];
};
