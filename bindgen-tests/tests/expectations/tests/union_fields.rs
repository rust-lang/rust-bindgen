#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union nsStyleUnion {
    pub mInt: ::std::os::raw::c_int,
    pub mFloat: f32,
    pub mPointer: *mut ::std::os::raw::c_void,
}
const _: () = {
    assert!(::std::mem::size_of::<nsStyleUnion>() == 8usize, "Size of nsStyleUnion");
    assert!(
        ::std::mem::align_of::<nsStyleUnion>() == 8usize,
        "Alignment of nsStyleUnion",
    );
    assert!(
        ::std::mem::offset_of!(nsStyleUnion, mInt) == 0usize,
        "Offset of field: nsStyleUnion::mInt",
    );
    assert!(
        ::std::mem::offset_of!(nsStyleUnion, mFloat) == 0usize,
        "Offset of field: nsStyleUnion::mFloat",
    );
    assert!(
        ::std::mem::offset_of!(nsStyleUnion, mPointer) == 0usize,
        "Offset of field: nsStyleUnion::mPointer",
    );
};
impl Default for nsStyleUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
