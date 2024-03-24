extern "C" {
    pub static mut foo: ::std::option::Option<
        unsafe extern "C" fn(
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub fn bar(a: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bar2(b: *const ::std::os::raw::c_char) -> f32;
}
pub type Char = ::std::os::raw::c_char;
pub type SChar = ::std::os::raw::c_schar;
pub type UChar = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Test {
    pub ch: ::std::os::raw::c_char,
    pub u: ::std::os::raw::c_uchar,
    pub d: ::std::os::raw::c_schar,
    pub cch: ::std::os::raw::c_char,
    pub cu: ::std::os::raw::c_uchar,
    pub cd: ::std::os::raw::c_schar,
    pub Cch: Char,
    pub Cu: UChar,
    pub Cd: SChar,
    pub Ccch: Char,
    pub Ccu: UChar,
    pub Ccd: SChar,
}
const _: () = {
    assert!(::std::mem::size_of::<Test>() == 12usize, "Size of Test");
    assert!(::std::mem::align_of::<Test>() == 1usize, "Alignment of Test");
    assert!(::std::mem::offset_of!(Test, ch) == 0usize, "Offset of field: Test::ch");
    assert!(::std::mem::offset_of!(Test, u) == 1usize, "Offset of field: Test::u");
    assert!(::std::mem::offset_of!(Test, d) == 2usize, "Offset of field: Test::d");
    assert!(::std::mem::offset_of!(Test, cch) == 3usize, "Offset of field: Test::cch");
    assert!(::std::mem::offset_of!(Test, cu) == 4usize, "Offset of field: Test::cu");
    assert!(::std::mem::offset_of!(Test, cd) == 5usize, "Offset of field: Test::cd");
    assert!(::std::mem::offset_of!(Test, Cch) == 6usize, "Offset of field: Test::Cch");
    assert!(::std::mem::offset_of!(Test, Cu) == 7usize, "Offset of field: Test::Cu");
    assert!(::std::mem::offset_of!(Test, Cd) == 8usize, "Offset of field: Test::Cd");
    assert!(::std::mem::offset_of!(Test, Ccch) == 9usize, "Offset of field: Test::Ccch");
    assert!(::std::mem::offset_of!(Test, Ccu) == 10usize, "Offset of field: Test::Ccu");
    assert!(::std::mem::offset_of!(Test, Ccd) == 11usize, "Offset of field: Test::Ccd");
};
