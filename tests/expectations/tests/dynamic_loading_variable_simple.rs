#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate libloading;
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: Result<*mut ::std::os::raw::c_int, ::libloading::Error>,
    pub baz: Result<*mut *mut ::std::os::raw::c_int, ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
        where
            P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(
        library: L,
    ) -> Result<Self, ::libloading::Error>
        where
            L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let foo = __library
            .get::<*mut ::std::os::raw::c_int>(b"foo\0")
            .map(|sym| *sym);
        let baz = __library
            .get::<*mut *mut ::std::os::raw::c_int>(b"baz\0")
            .map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            baz,
        })
    }
    pub unsafe fn get_var_foo(&self) -> *mut ::std::os::raw::c_int {
        *self.foo.as_ref().expect("Expected variable, got error.")
    }
    pub unsafe fn get_var_baz(&self) -> *mut *mut ::std::os::raw::c_int {
        *self.baz.as_ref().expect("Expected variable, got error.")
    }
}
