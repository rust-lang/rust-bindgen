#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        unsafe { ::std::slice::from_raw_parts(self.as_ptr(), len) }
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        unsafe { ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len) }
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
impl Default for C_with_zero_length_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
impl Default for C_with_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug)]
pub struct C_with_zero_length_array_and_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
impl Default for C_with_zero_length_array_and_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_and_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IncompleteArrayNonCopiable {
    pub whatever: *mut ::std::os::raw::c_void,
    pub incomplete_array: __IncompleteArrayField<C>,
}
impl Default for IncompleteArrayNonCopiable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub d: f32,
    pub i: ::std::os::raw::c_int,
}
impl Default for Union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithUnion {
    pub data: Union,
}
impl Default for WithUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
extern "C" {
    #[link_name = "\u{1}_ZNK32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar(
        this: *const RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar1(
        this: *mut RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEi"]
    pub fn RealAbstractionWithTonsOfMethods_bar2(
        this: *mut RealAbstractionWithTonsOfMethods,
        foo: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3staEv"]
    pub fn RealAbstractionWithTonsOfMethods_sta();
}
impl RealAbstractionWithTonsOfMethods {
    #[inline]
    pub unsafe fn bar(&self) {
        unsafe { RealAbstractionWithTonsOfMethods_bar(self) }
    }
    #[inline]
    pub unsafe fn bar1(&mut self) {
        unsafe { RealAbstractionWithTonsOfMethods_bar1(self) }
    }
    #[inline]
    pub unsafe fn bar2(&mut self, foo: ::std::os::raw::c_int) {
        unsafe { RealAbstractionWithTonsOfMethods_bar2(self, foo) }
    }
    #[inline]
    pub unsafe fn sta() {
        unsafe { RealAbstractionWithTonsOfMethods_sta() }
    }
}
