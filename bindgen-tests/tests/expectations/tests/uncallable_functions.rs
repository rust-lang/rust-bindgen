#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct Test__bindgen_vtable {
    pub Test_a: unsafe extern "C" fn(this: *mut Test),
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Test {
    pub vtable_: *const Test__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Test"][::std::mem::size_of::<Test>() - 8usize];
    ["Alignment of Test"][::std::mem::align_of::<Test>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN4Test1bEv"]
    pub fn Test_b(this: *mut Test);
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN4Test1cEv"]
    pub fn Test_c(this: *mut Test);
}
impl Default for Test {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Test {
    #[inline]
    pub unsafe fn b(&mut self) {
        Test_b(self)
    }
    #[inline]
    pub unsafe fn c(&mut self) {
        Test_c(self)
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN4Test1aEv"]
    pub fn Test_a(this: *mut ::std::os::raw::c_void);
}
