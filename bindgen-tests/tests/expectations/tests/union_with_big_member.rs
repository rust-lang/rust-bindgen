#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_int; 33usize],
}
const _: () = {
    assert!(::std::mem::size_of::<WithBigArray>() == 132usize, "Size of WithBigArray");
    assert!(
        ::std::mem::align_of::<WithBigArray>() == 4usize,
        "Alignment of WithBigArray",
    );
    assert!(
        ::std::mem::offset_of!(WithBigArray, a) == 0usize,
        "Offset of field: WithBigArray::a",
    );
    assert!(
        ::std::mem::offset_of!(WithBigArray, b) == 0usize,
        "Offset of field: WithBigArray::b",
    );
};
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigArray2 {
    pub a: ::std::os::raw::c_int,
    pub b: [::std::os::raw::c_char; 33usize],
}
const _: () = {
    assert!(::std::mem::size_of::<WithBigArray2>() == 36usize, "Size of WithBigArray2");
    assert!(
        ::std::mem::align_of::<WithBigArray2>() == 4usize,
        "Alignment of WithBigArray2",
    );
    assert!(
        ::std::mem::offset_of!(WithBigArray2, a) == 0usize,
        "Offset of field: WithBigArray2::a",
    );
    assert!(
        ::std::mem::offset_of!(WithBigArray2, b) == 0usize,
        "Offset of field: WithBigArray2::b",
    );
};
impl Default for WithBigArray2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithBigMember {
    pub a: ::std::os::raw::c_int,
    pub b: WithBigArray,
}
const _: () = {
    assert!(::std::mem::size_of::<WithBigMember>() == 132usize, "Size of WithBigMember");
    assert!(
        ::std::mem::align_of::<WithBigMember>() == 4usize,
        "Alignment of WithBigMember",
    );
    assert!(
        ::std::mem::offset_of!(WithBigMember, a) == 0usize,
        "Offset of field: WithBigMember::a",
    );
    assert!(
        ::std::mem::offset_of!(WithBigMember, b) == 0usize,
        "Offset of field: WithBigMember::b",
    );
};
impl Default for WithBigMember {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
