#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        1usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        1usize,
        concat!("Alignment of ", stringify!(Bar))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3BarC1Ev"]
    pub fn Bar_Bar(this: *mut Bar);
}
impl Bar {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bar_Bar(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
struct Box_Bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Bar {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(1usize, 1usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            Bar_Bar(ret.ptr as *mut Bar);
            ret
        }
    }
}
impl Drop for Box_Bar {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
