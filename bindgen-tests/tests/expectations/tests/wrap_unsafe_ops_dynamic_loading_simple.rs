#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
        unsafe extern "C" fn(x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub baz: Result<
        unsafe extern "C" fn() -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub FLUX: Result<*mut ::std::os::raw::c_int, ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = unsafe { ::libloading::Library::new(path) }?;
        unsafe { Self::from_library(library) }
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let foo = unsafe { __library.get(b"foo\0") }.map(|sym| *sym);
        let bar = unsafe { __library.get(b"bar\0") }.map(|sym| *sym);
        let baz = unsafe { __library.get(b"baz\0") }.map(|sym| *sym);
        let FLUX = unsafe { __library.get::<*mut ::std::os::raw::c_int>(b"FLUX\0") }
            .map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            bar,
            baz,
            FLUX,
        })
    }
    pub unsafe fn foo(
        &self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        unsafe { (self.foo.as_ref().expect("Expected function, got error."))(x, y) }
    }
    pub unsafe fn bar(&self, x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {
        unsafe { (self.bar.as_ref().expect("Expected function, got error."))(x) }
    }
    pub unsafe fn baz(&self) -> ::std::os::raw::c_int {
        unsafe { (self.baz.as_ref().expect("Expected function, got error."))() }
    }
    pub unsafe fn FLUX(&self) -> *mut ::std::os::raw::c_int {
        *self.FLUX.as_ref().expect("Expected variable, got error.")
    }
}
