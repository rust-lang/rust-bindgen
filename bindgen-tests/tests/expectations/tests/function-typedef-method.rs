#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type Function = ::std::option::Option<
    unsafe extern "C" fn(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
>;
pub type FunctionAlias = Function;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct First {
    pub value: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of First"][::std::mem::size_of::<First>() - 4usize];
    ["Alignment of First"][::std::mem::align_of::<First>() - 4usize];
    ["Offset of field: First::value"][::std::mem::offset_of!(First, value) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN5First6methodEi"]
    pub fn First_method(
        this: *mut First,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN5First13static_methodEi"]
    pub fn First_static_method(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
impl First {
    #[inline]
    pub unsafe fn method(
        &mut self,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        First_method(self, arg1)
    }
    #[inline]
    pub unsafe fn static_method(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
        First_static_method(arg1)
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Second {
    pub value: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Second"][::std::mem::size_of::<Second>() - 4usize];
    ["Alignment of Second"][::std::mem::align_of::<Second>() - 4usize];
    ["Offset of field: Second::value"][::std::mem::offset_of!(Second, value) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN6Second6methodEi"]
    pub fn Second_method(
        this: *mut Second,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
impl Second {
    #[inline]
    pub unsafe fn method(
        &mut self,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        Second_method(self, arg1)
    }
}
#[repr(C)]
pub struct Virtual__bindgen_vtable {
    pub Virtual_method: unsafe extern "C" fn(
        this: *mut Virtual,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Virtual {
    pub vtable_: *const Virtual__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Virtual"][::std::mem::size_of::<Virtual>() - 8usize];
    ["Alignment of Virtual"][::std::mem::align_of::<Virtual>() - 8usize];
};
impl Default for Virtual {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    #[link_name = "\u{1}_ZN7Virtual6methodEi"]
    pub fn Virtual_method(
        this: *mut ::std::os::raw::c_void,
        arg1: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}_Z13free_functioni"]
    pub fn free_function(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
