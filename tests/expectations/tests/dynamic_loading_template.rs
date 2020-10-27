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
        unsafe extern "C" fn(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    foo1: Result<unsafe extern "C" fn(x: f32) -> f32, ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let foo = __library.get("foo".as_bytes()).map(|sym| *sym);
        let foo1 = __library.get("foo1".as_bytes()).map(|sym| *sym);
        Ok(TestLib {
            __library: __library,
            foo,
            foo1,
        })
    }
    pub fn can_call(&self) -> CheckTestLib {
        CheckTestLib { __library: self }
    }
    pub unsafe fn foo(
        &self,
        x: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        let sym = self.foo.as_ref().expect("Expected function, got error.");
        (sym)(x)
    }
    pub unsafe fn foo1(&self, x: f32) -> f32 {
        let sym = self.foo1.as_ref().expect("Expected function, got error.");
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
    pub fn foo1(&self) -> Result<(), &'a ::libloading::Error> {
        self.__library.foo1.as_ref().map(|_| ())
    }
}
