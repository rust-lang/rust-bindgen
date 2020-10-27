#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate libloading;
pub struct TestLib {
    __library: ::libloading::Library,
    foo: Result<
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    baz: Result<
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let foo = __library.get("foo".as_bytes()).map(|sym| *sym);
        let baz = __library.get("baz".as_bytes()).map(|sym| *sym);
        Ok(TestLib {
            __library: __library,
            foo,
            baz,
        })
    }
    pub fn can_call(&self) -> CheckTestLib {
        CheckTestLib { __library: self }
    }
    pub unsafe fn foo(
        &self,
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let sym = self.foo.as_ref().expect("Expected function, got error.");
        (sym)(x)
    }
    pub unsafe fn baz(
        &self,
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let sym = self.baz.as_ref().expect("Expected function, got error.");
        (sym)(x)
    }
}
pub struct CheckTestLib<'a> {
    __library: &'a TestLib,
}
impl<'a> CheckTestLib<'a> {
    pub fn foo(&self) -> Result<(), &'a ::libloading::Error> {
        self.__library.foo.as_ref().map(|_| ())
    }
    pub fn baz(&self) -> Result<(), &'a ::libloading::Error> {
        self.__library.baz.as_ref().map(|_| ())
    }
}
