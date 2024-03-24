#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Copy, Clone, Hash, Debug, Default)]
#[repr(C)]
pub struct __BindgenComplex<T> {
    pub re: T,
    pub im: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub bar: ::std::os::raw::c_float,
    pub baz: ::std::os::raw::c_float,
    pub bazz: ::std::os::raw::c_double,
    pub bazzz: *mut u128,
    pub complexFloat: __BindgenComplex<::std::os::raw::c_float>,
    pub complexDouble: __BindgenComplex<::std::os::raw::c_double>,
}
const _: () = {
    assert!(::std::mem::size_of::<foo>() == 48usize, "Size of foo");
    assert!(::std::mem::align_of::<foo>() == 8usize, "Alignment of foo");
    assert!(::std::mem::offset_of!(foo, bar) == 0usize, "Offset of field: foo::bar");
    assert!(::std::mem::offset_of!(foo, baz) == 4usize, "Offset of field: foo::baz");
    assert!(::std::mem::offset_of!(foo, bazz) == 8usize, "Offset of field: foo::bazz");
    assert!(
        ::std::mem::offset_of!(foo, bazzz) == 16usize,
        "Offset of field: foo::bazzz",
    );
    assert!(
        ::std::mem::offset_of!(foo, complexFloat) == 24usize,
        "Offset of field: foo::complexFloat",
    );
    assert!(
        ::std::mem::offset_of!(foo, complexDouble) == 32usize,
        "Offset of field: foo::complexDouble",
    );
};
impl Default for foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
