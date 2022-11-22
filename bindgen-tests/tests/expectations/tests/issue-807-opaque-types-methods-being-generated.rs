#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Pupper {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Pupper() {
    assert_eq!(
        ::std::mem::size_of::<Pupper>(),
        1usize,
        concat!("Size of: ", stringify!(Pupper))
    );
    assert_eq!(
        ::std::mem::align_of::<Pupper>(),
        1usize,
        concat!("Alignment of ", stringify!(Pupper))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Doggo {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Doggo() {
    assert_eq!(
        ::std::mem::size_of::<Doggo>(),
        1usize,
        concat!("Size of: ", stringify!(Doggo))
    );
    assert_eq!(
        ::std::mem::align_of::<Doggo>(),
        1usize,
        concat!("Alignment of ", stringify!(Doggo))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SuchWow {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_SuchWow() {
    assert_eq!(
        ::std::mem::size_of::<SuchWow>(),
        1usize,
        concat!("Size of: ", stringify!(SuchWow))
    );
    assert_eq!(
        ::std::mem::align_of::<SuchWow>(),
        1usize,
        concat!("Alignment of ", stringify!(SuchWow))
    );
}
#[repr(C)]
#[repr(align(1))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Opaque {
    pub _bindgen_opaque_blob: u8,
}
#[test]
fn bindgen_test_layout_Opaque() {
    assert_eq!(
        ::std::mem::size_of::<Opaque>(),
        1usize,
        concat!("Size of: ", stringify!(Opaque))
    );
    assert_eq!(
        ::std::mem::align_of::<Opaque>(),
        1usize,
        concat!("Alignment of ", stringify!(Opaque))
    );
}
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
#[test]
fn bindgen_test_layout_Allowlisted() {
    const UNINIT: ::std::mem::MaybeUninit<Allowlisted> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Allowlisted>(),
        1usize,
        concat!("Size of: ", stringify!(Allowlisted))
    );
    assert_eq!(
        ::std::mem::align_of::<Allowlisted>(),
        1usize,
        concat!("Alignment of ", stringify!(Allowlisted))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).some_member) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Allowlisted),
            "::",
            stringify!(some_member)
        )
    );
}
