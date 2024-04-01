#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub m_baz: ::std::os::raw::c_int,
}
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 4usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 4usize];
    ["Offset of field: Bar::m_baz"][::std::mem::offset_of!(Bar, m_baz) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN3BarC1Ei"]
    pub fn Bar_Bar(this: *mut Bar, baz: ::std::os::raw::c_int);
}
impl Bar {
    #[inline]
    pub unsafe fn new(baz: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        Bar_Bar(__bindgen_tmp.as_mut_ptr(), baz);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Baz {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZN3Baz3FOOE"]
    pub static Baz_FOO: [Bar; 0usize];
}
const _: () = {
    ["Size of Baz"][::std::mem::size_of::<Baz>() - 1usize];
    ["Alignment of Baz"][::std::mem::align_of::<Baz>() - 1usize];
};
