#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct PureVirtualIFace__bindgen_vtable {
    pub PureVirtualIFace_Foo: unsafe extern "C" fn(this: *mut PureVirtualIFace),
    pub PureVirtualIFace_Bar: unsafe extern "C" fn(
        this: *mut PureVirtualIFace,
        arg1: ::std::os::raw::c_uint,
    ),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PureVirtualIFace {
    pub vtable_: *const PureVirtualIFace__bindgen_vtable,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<PureVirtualIFace>() == 8usize,
        "Size of PureVirtualIFace",
    );
    assert!(
        ::std::mem::align_of::<PureVirtualIFace>() == 8usize,
        "Alignment of PureVirtualIFace",
    );
};
impl Default for PureVirtualIFace {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct AnotherInterface__bindgen_vtable {
    pub AnotherInterface_Baz: unsafe extern "C" fn(this: *mut AnotherInterface),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AnotherInterface {
    pub vtable_: *const AnotherInterface__bindgen_vtable,
}
const _: () = {
    assert!(
        ::std::mem::size_of::<AnotherInterface>() == 8usize,
        "Size of AnotherInterface",
    );
    assert!(
        ::std::mem::align_of::<AnotherInterface>() == 8usize,
        "Alignment of AnotherInterface",
    );
};
impl Default for AnotherInterface {
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
pub struct Implementation {
    pub _base: PureVirtualIFace,
}
const _: () = {
    assert!(::std::mem::size_of::<Implementation>() == 8usize, "Size of Implementation");
    assert!(
        ::std::mem::align_of::<Implementation>() == 8usize,
        "Alignment of Implementation",
    );
};
impl Default for Implementation {
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
pub struct DoubleImpl {
    pub _base: PureVirtualIFace,
    pub _base_1: AnotherInterface,
}
const _: () = {
    assert!(::std::mem::size_of::<DoubleImpl>() == 16usize, "Size of DoubleImpl");
    assert!(::std::mem::align_of::<DoubleImpl>() == 8usize, "Alignment of DoubleImpl");
};
impl Default for DoubleImpl {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
