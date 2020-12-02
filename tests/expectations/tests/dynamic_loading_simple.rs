#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate libloading;
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: Result<
        unsafe extern "C" fn(
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub bar: Result<
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub baz: Result<
        unsafe extern "C" fn() -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let foo = __library.get(b"foo\0").map(|sym| *sym);
        let bar = __library.get(b"bar\0").map(|sym| *sym);
        let baz = __library.get(b"baz\0").map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            bar,
            baz,
        })
    }
    pub unsafe fn foo(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let sym = self.foo.as_ref().expect("Expected function, got error.");
        (sym)(x, y)
    }
    pub unsafe fn bar(
        &self,
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let sym = self.bar.as_ref().expect("Expected function, got error.");
        (sym)(x)
    }
    pub unsafe fn baz(&self) -> ::std::os::raw::c_int {
        let sym = self.baz.as_ref().expect("Expected function, got error.");
        (sym)()
    }
}
