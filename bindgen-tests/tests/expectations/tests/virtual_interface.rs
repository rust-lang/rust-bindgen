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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of PureVirtualIFace"][::std::mem::size_of::<PureVirtualIFace>() - 8usize];
    [
        "Alignment of PureVirtualIFace",
    ][::std::mem::align_of::<PureVirtualIFace>() - 8usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AnotherInterface"][::std::mem::size_of::<AnotherInterface>() - 8usize];
    [
        "Alignment of AnotherInterface",
    ][::std::mem::align_of::<AnotherInterface>() - 8usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Implementation"][::std::mem::size_of::<Implementation>() - 8usize];
    ["Alignment of Implementation"][::std::mem::align_of::<Implementation>() - 8usize];
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DoubleImpl"][::std::mem::size_of::<DoubleImpl>() - 16usize];
    ["Alignment of DoubleImpl"][::std::mem::align_of::<DoubleImpl>() - 8usize];
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
