#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern "C" {
    #[link_name = "\u{1}_Z3fooPKcz"]
    pub fn foo(fmt: *const ::std::os::raw::c_char, ...);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        1usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        1usize,
        concat!("Alignment of ", stringify!(Bar))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN3Bar3fooEPKcz"]
    pub fn Bar_foo(this: *mut Bar, fmt: *const ::std::os::raw::c_char, ...);
}
impl Bar {
    #[inline]
    pub unsafe fn foo(
        &mut self,
        fmt: *const ::std::os::raw::c_char,
        var_args: impl VarArgs,
    ) {
        var_args.call_Bar_foo(self, fmt)
    }
}
pub trait VarArgs {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    );
}
impl VarArgs for () {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let () = self;
        Bar_foo(this, fmt)
    }
}
impl<T0> VarArgs for (T0,) {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let (t0,) = self;
        Bar_foo(this, fmt, t0)
    }
}
impl<T0, T1> VarArgs for (T0, T1) {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let (t0, t1) = self;
        Bar_foo(this, fmt, t0, t1)
    }
}
impl<T0, T1, T2> VarArgs for (T0, T1, T2) {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let (t0, t1, t2) = self;
        Bar_foo(this, fmt, t0, t1, t2)
    }
}
impl<T0, T1, T2, T3> VarArgs for (T0, T1, T2, T3) {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let (t0, t1, t2, t3) = self;
        Bar_foo(this, fmt, t0, t1, t2, t3)
    }
}
impl<T0, T1, T2, T3, T4> VarArgs for (T0, T1, T2, T3, T4) {
    unsafe fn call_Bar_foo(
        self,
        this: *mut Bar,
        fmt: *const ::std::os::raw::c_char,
    ) {
        let (t0, t1, t2, t3, t4) = self;
        Bar_foo(this, fmt, t0, t1, t2, t3, t4)
    }
}
