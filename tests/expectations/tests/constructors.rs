#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TestOverload {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_TestOverload() {
    assert_eq!(
        ::std::mem::size_of::<TestOverload>(),
        1usize,
        concat!("Size of: ", stringify!(TestOverload))
    );
    assert_eq!(
        ::std::mem::align_of::<TestOverload>(),
        1usize,
        concat!("Alignment of ", stringify!(TestOverload))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN12TestOverloadC1Ei"]
    pub fn TestOverload_TestOverload(
        this: *mut TestOverload,
        arg1: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN12TestOverloadC1Ed"]
    pub fn TestOverload_TestOverload1(this: *mut TestOverload, arg1: f64);
}
impl TestOverload {
    #[inline]
    pub unsafe fn new(arg1: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        TestOverload_TestOverload(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(arg1: f64) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        TestOverload_TestOverload1(__bindgen_tmp.as_mut_ptr(), arg1);
        __bindgen_tmp.assume_init()
    }
}
struct Box_TestOverload {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_TestOverload {
    #[inline]
    pub fn new(arg1: ::std::os::raw::c_int) -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            TestOverload_TestOverload(ret.ptr as *mut TestOverload, arg1);
            ret
        }
    }
    #[inline]
    pub fn new1(arg1: f64) -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            TestOverload_TestOverload1(ret.ptr as *mut TestOverload, arg1);
            ret
        }
    }
}
impl Drop for Box_TestOverload {
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
#[derive(Debug, Default, Copy, Clone)]
pub struct TestPublicNoArgs {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_TestPublicNoArgs() {
    assert_eq!(
        ::std::mem::size_of::<TestPublicNoArgs>(),
        1usize,
        concat!("Size of: ", stringify!(TestPublicNoArgs))
    );
    assert_eq!(
        ::std::mem::align_of::<TestPublicNoArgs>(),
        1usize,
        concat!("Alignment of ", stringify!(TestPublicNoArgs))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN16TestPublicNoArgsC1Ev"]
    pub fn TestPublicNoArgs_TestPublicNoArgs(this: *mut TestPublicNoArgs);
}
impl TestPublicNoArgs {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        TestPublicNoArgs_TestPublicNoArgs(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
struct Box_TestPublicNoArgs {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_TestPublicNoArgs {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            TestPublicNoArgs_TestPublicNoArgs(ret.ptr as *mut TestPublicNoArgs);
            ret
        }
    }
}
impl Drop for Box_TestPublicNoArgs {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
