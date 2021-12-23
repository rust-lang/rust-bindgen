#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate libloading;
pub struct TestLib {
    __library: ::libloading::Library,
    pub foo: unsafe extern "C" fn(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub bar: unsafe extern "C" fn(
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
    pub baz: unsafe extern "C" fn() -> ::std::os::raw::c_int,
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
        let foo = __library.get(b"foo\0").map(|sym| *sym)?;
        let bar = __library.get(b"bar\0").map(|sym| *sym)?;
        let baz = __library.get(b"baz\0").map(|sym| *sym)?;
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
        (self.foo)(x, y)
    }
    pub unsafe fn bar(
        &self,
        x: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        (self.bar)(x)
    }
    pub unsafe fn baz(&self) -> ::std::os::raw::c_int {
        (self.baz)()
    }
}
