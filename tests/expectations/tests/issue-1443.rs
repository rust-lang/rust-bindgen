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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.f);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(f))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Bar>() };
            let struct_ptr = &struct_instance as *const Bar;
            let field_ptr = std::ptr::addr_of!(struct_instance.m);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Bar), "::", stringify!(m))
    );
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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Baz>() };
            let struct_ptr = &struct_instance as *const Baz;
            let field_ptr = std::ptr::addr_of!(struct_instance.f);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Baz), "::", stringify!(f))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Baz>() };
            let struct_ptr = &struct_instance as *const Baz;
            let field_ptr = std::ptr::addr_of!(struct_instance.m);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Baz), "::", stringify!(m))
    );
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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Tar>() };
            let struct_ptr = &struct_instance as *const Tar;
            let field_ptr = std::ptr::addr_of!(struct_instance.f);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Tar), "::", stringify!(f))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Tar>() };
            let struct_ptr = &struct_instance as *const Tar;
            let field_ptr = std::ptr::addr_of!(struct_instance.m);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Tar), "::", stringify!(m))
    );
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
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Taz>() };
            let struct_ptr = &struct_instance as *const Taz;
            let field_ptr = std::ptr::addr_of!(struct_instance.f);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Taz), "::", stringify!(f))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Taz>() };
            let struct_ptr = &struct_instance as *const Taz;
            let field_ptr = std::ptr::addr_of!(struct_instance.m);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Taz), "::", stringify!(m))
    );
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
