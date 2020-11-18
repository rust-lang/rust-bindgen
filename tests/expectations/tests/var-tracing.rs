#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub m_baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Bar>())).m_baz as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Bar),
            "::",
            stringify!(m_baz)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3BarC1Ei"]
    pub fn Bar_Bar(this: *mut Bar, baz: ::std::os::raw::c_int);
}
impl Bar {
    #[inline]
    pub unsafe fn new(baz: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bar_Bar(__bindgen_tmp.as_mut_ptr(), baz);
        __bindgen_tmp.assume_init()
    }
}
struct Box_Bar {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Bar {
    #[inline]
    pub fn new(baz: ::std::os::raw::c_int) -> Self {
        unsafe {
            let ret = Self {
                ptr: ::std::alloc::alloc(
                    ::std::alloc::Layout::from_size_align(4usize, 4usize)
                        .unwrap(),
                ) as *mut ::std::ffi::c_void,
            };
            Bar_Bar(ret.ptr as *mut Bar, baz);
            ret
        }
    }
}
impl Drop for Box_Bar {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN3Baz3FOOE"]
    pub static mut Baz_FOO: [Bar; 0usize];
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        1usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        1usize,
        concat!("Alignment of ", stringify!(Baz))
    );
}
struct Box_Baz {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_Baz {}
impl Drop for Box_Baz {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(1usize, 1usize).unwrap(),
            );
        }
    }
}
