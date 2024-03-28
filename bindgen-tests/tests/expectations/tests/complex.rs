#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TestDouble {
    pub mMember: __BindgenComplex<f64>,
}
const _: () = {
    assert!(::std::mem::size_of::<TestDouble>() == 16usize, "Size of TestDouble");
    assert!(::std::mem::align_of::<TestDouble>() == 8usize, "Alignment of TestDouble");
    assert!(
        ::std::mem::offset_of!(TestDouble, mMember) == 0usize,
        "Offset of field: TestDouble::mMember",
    );
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestDoublePtr {
    pub mMember: *mut __BindgenComplex<f64>,
}
const _: () = {
    assert!(::std::mem::size_of::<TestDoublePtr>() == 8usize, "Size of TestDoublePtr");
    assert!(
        ::std::mem::align_of::<TestDoublePtr>() == 8usize,
        "Alignment of TestDoublePtr",
    );
    assert!(
        ::std::mem::offset_of!(TestDoublePtr, mMember) == 0usize,
        "Offset of field: TestDoublePtr::mMember",
    );
};
impl Default for TestDoublePtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TestFloat {
    pub mMember: __BindgenComplex<f32>,
}
const _: () = {
    assert!(::std::mem::size_of::<TestFloat>() == 8usize, "Size of TestFloat");
    assert!(::std::mem::align_of::<TestFloat>() == 4usize, "Alignment of TestFloat");
    assert!(
        ::std::mem::offset_of!(TestFloat, mMember) == 0usize,
        "Offset of field: TestFloat::mMember",
    );
};
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct TestFloatPtr {
    pub mMember: *mut __BindgenComplex<f32>,
}
const _: () = {
    assert!(::std::mem::size_of::<TestFloatPtr>() == 8usize, "Size of TestFloatPtr");
    assert!(
        ::std::mem::align_of::<TestFloatPtr>() == 8usize,
        "Alignment of TestFloatPtr",
    );
    assert!(
        ::std::mem::offset_of!(TestFloatPtr, mMember) == 0usize,
        "Offset of field: TestFloatPtr::mMember",
    );
};
impl Default for TestFloatPtr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
