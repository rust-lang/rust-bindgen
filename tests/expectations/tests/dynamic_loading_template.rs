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
        unsafe extern "C" fn(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
        ::libloading::Error,
    >,
    pub foo1: Result<unsafe extern "C" fn(x: f32) -> f32, ::libloading::Error>,
}
impl TestLib {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let __library = ::libloading::Library::new(path)?;
        let foo = __library.get(b"foo\0").map(|sym| *sym);
        let foo1 = __library.get(b"foo1\0").map(|sym| *sym);
        Ok(TestLib {
            __library,
            foo,
            foo1,
        })
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
