#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub f: *const Foo,
    pub m: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        16usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        8usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Bar), "::", stringify!(f))
        );
    }
    test_field_f();
    fn test_field_m() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize
            },
            8usize,
            concat!("Offset of field: ", stringify!(Bar), "::", stringify!(m))
        );
    }
    test_field_m();
}
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Baz {
    pub f: *mut Foo,
    pub m: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Baz() {
    assert_eq!(
        ::std::mem::size_of::<Baz>(),
        16usize,
        concat!("Size of: ", stringify!(Baz))
    );
    assert_eq!(
        ::std::mem::align_of::<Baz>(),
        8usize,
        concat!("Alignment of ", stringify!(Baz))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Baz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Baz), "::", stringify!(f))
        );
    }
    test_field_f();
    fn test_field_m() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Baz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize
            },
            8usize,
            concat!("Offset of field: ", stringify!(Baz), "::", stringify!(m))
        );
    }
    test_field_m();
}
impl Default for Baz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Tar {
    pub f: *const Foo,
    pub m: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Tar() {
    assert_eq!(
        ::std::mem::size_of::<Tar>(),
        16usize,
        concat!("Size of: ", stringify!(Tar))
    );
    assert_eq!(
        ::std::mem::align_of::<Tar>(),
        8usize,
        concat!("Alignment of ", stringify!(Tar))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Tar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Tar), "::", stringify!(f))
        );
    }
    test_field_f();
    fn test_field_m() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Tar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize
            },
            8usize,
            concat!("Offset of field: ", stringify!(Tar), "::", stringify!(m))
        );
    }
    test_field_m();
}
impl Default for Tar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Taz {
    pub f: *mut Foo,
    pub m: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_Taz() {
    assert_eq!(
        ::std::mem::size_of::<Taz>(),
        16usize,
        concat!("Size of: ", stringify!(Taz))
    );
    assert_eq!(
        ::std::mem::align_of::<Taz>(),
        8usize,
        concat!("Alignment of ", stringify!(Taz))
    );
    fn test_field_f() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Taz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(Taz), "::", stringify!(f))
        );
    }
    test_field_f();
    fn test_field_m() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Taz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).m) as usize - ptr as usize
            },
            8usize,
            concat!("Offset of field: ", stringify!(Taz), "::", stringify!(m))
        );
    }
    test_field_m();
}
impl Default for Taz {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
