extern "C" {
    pub static mut foo: ::std::option::Option<
        unsafe extern "C" fn(
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 12usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 1usize];
    ["Offset of field: Test::ch"][::std::mem::offset_of!(Test, ch) - 0usize];
    ["Offset of field: Test::u"][::std::mem::offset_of!(Test, u) - 1usize];
    ["Offset of field: Test::d"][::std::mem::offset_of!(Test, d) - 2usize];
    ["Offset of field: Test::cch"][::std::mem::offset_of!(Test, cch) - 3usize];
    ["Offset of field: Test::cu"][::std::mem::offset_of!(Test, cu) - 4usize];
    ["Offset of field: Test::cd"][::std::mem::offset_of!(Test, cd) - 5usize];
    ["Offset of field: Test::Cch"][::std::mem::offset_of!(Test, Cch) - 6usize];
    ["Offset of field: Test::Cu"][::std::mem::offset_of!(Test, Cu) - 7usize];
    ["Offset of field: Test::Cd"][::std::mem::offset_of!(Test, Cd) - 8usize];
    ["Offset of field: Test::Ccch"][::std::mem::offset_of!(Test, Ccch) - 9usize];
    ["Offset of field: Test::Ccu"][::std::mem::offset_of!(Test, Ccu) - 10usize];
    ["Offset of field: Test::Ccd"][::std::mem::offset_of!(Test, Ccd) - 11usize];
};
