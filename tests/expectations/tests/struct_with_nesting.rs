#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub a: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union foo__bindgen_ty_1 {
    pub b: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: foo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: foo__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_1 {
    pub c1: ::std::os::raw::c_ushort,
    pub c2: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    fn test_field_c1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c1) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(c1)
            )
        );
    }
    test_field_c1();
    fn test_field_c2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c2) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(c2)
            )
        );
    }
    test_field_c2();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_2 {
    pub d1: ::std::os::raw::c_uchar,
    pub d2: ::std::os::raw::c_uchar,
    pub d3: ::std::os::raw::c_uchar,
    pub d4: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    fn test_field_d1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d1) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(d1)
            )
        );
    }
    test_field_d1();
    fn test_field_d2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d2) as usize - ptr as usize
            },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(d2)
            )
        );
    }
    test_field_d2();
    fn test_field_d3() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d3) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(d3)
            )
        );
    }
    test_field_d3();
    fn test_field_d4() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d4) as usize - ptr as usize
            },
            3usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(d4)
            )
        );
    }
    test_field_d4();
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<foo__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
impl Default for foo__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        8usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
        );
    }
    test_field_a();
}
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
