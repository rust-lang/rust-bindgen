#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: Result<
        unsafe extern "C" fn(x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub baz: Result<
        unsafe extern "C" fn(x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub bazz: Result<
        unsafe extern "C" fn(arg1: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
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
        let baz = __library.get(b"baz\0").map(|sym| *sym);
        let bazz = __library.get(b"bazz\0").map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            baz,
            bazz,
        })
    }
    pub unsafe fn foo(&self, x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {
        (self.foo.as_ref().expect("Expected function, got error."))(x)
    }
    pub unsafe fn baz(&self, x: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int {
        (self.baz.as_ref().expect("Expected function, got error."))(x)
    }
}
