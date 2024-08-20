#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _x: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of A"][::std::mem::size_of::<A>() - 4usize];
    ["Alignment of A"][::std::mem::align_of::<A>() - 4usize];
    ["Offset of field: A::_x"][::std::mem::offset_of!(A, _x) - 0usize];
};
extern "C" {
    #[link_name = "\u{1}_ZN1A13some_functionEv"]
    pub fn A_some_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1A19some_other_functionEv"]
    pub fn A_some_other_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1AC1Ei"]
    pub fn A_A(this: *mut A, x: ::std::os::raw::c_int);
}
impl A {
    #[inline]
    pub unsafe fn some_function(&mut self) {
        A_some_function(self)
    }
    #[inline]
    pub unsafe fn some_other_function(&mut self) {
        A_some_other_function(self)
    }
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: Result<
        unsafe extern "C" fn(x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub bar: Result<unsafe extern "C" fn(), ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let foo = __library.get(b"foo\0").map(|sym| *sym);
        let bar = __library.get(b"bar\0").map(|sym| *sym);
        Ok(TestLib { __library, foo, bar })
    }
    pub unsafe fn foo(&self, x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {
        (self.foo.as_ref().expect("Expected function, got error."))(x)
    }
    pub unsafe fn bar(&self) {
        (self.bar.as_ref().expect("Expected function, got error."))()
    }
}
