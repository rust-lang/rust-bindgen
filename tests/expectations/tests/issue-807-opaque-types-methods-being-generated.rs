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
struct Box_Pupper {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Pupper {}
impl Drop for Box_Pupper {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
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
struct Box_Doggo {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Doggo {}
impl Drop for Box_Doggo {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
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
struct Box_SuchWow {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_SuchWow {}
impl Drop for Box_SuchWow {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
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
struct Box_Opaque {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Opaque {
    #[inline]
    pub fn eleven_out_of_ten(&mut self) -> SuchWow {
        unsafe { Opaque_eleven_out_of_ten(self.ptr as *mut Opaque) }
    }
    #[inline]
    pub fn new(pup: Pupper) -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            Opaque_Opaque(ret.ptr as *mut Opaque, pup);
            ret
        }
    }
}
impl Drop for Box_Opaque {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
extern "C" {
    #[link_name = "\u{1}_ZN6Opaque11MAJESTIC_AFE"]
    pub static mut Opaque_MAJESTIC_AF: Doggo;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Whitelisted {
    pub some_member: Opaque,
}
#[test]
fn bindgen_test_layout_Whitelisted() {
    assert_eq!(
        ::std::mem::size_of::<Whitelisted>(),
        1usize,
        concat!("Size of: ", stringify!(Whitelisted))
    );
    assert_eq!(
        ::std::mem::align_of::<Whitelisted>(),
        1usize,
        concat!("Alignment of ", stringify!(Whitelisted))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Whitelisted>())).some_member as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Whitelisted),
            "::",
            stringify!(some_member)
        )
    );
}
struct Box_Whitelisted {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Whitelisted {}
impl Drop for Box_Whitelisted {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
