#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[test]
fn test_mixed_union_pointer_size() {
    assert_eq!(::std::mem::size_of::<test_MixedCharPointer>(), 8);
}
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C, align(8))]
pub struct __BindgenOpaqueArray8<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray8<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
pub union test_MixedUnionPointer<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub val: ::std::mem::ManuallyDrop<T>,
    pub ptr: ::std::mem::ManuallyDrop<*mut test_MixedUnionPointer<T>>,
}
impl<T> Default for test_MixedUnionPointer<T> {
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
pub struct test_MixedUnionValue {
    pub _address: u8,
}
impl Default for test_MixedUnionValue {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type test_MixedCharPointer = test_MixedUnionPointer<::std::os::raw::c_char>;
pub type test_MixedCharDouble = __BindgenOpaqueArray8<[u8; 8usize]>;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: test_MixedUnionPointer_open0_char_int_close0",
    ][::std::mem::size_of::<test_MixedUnionPointer<::std::os::raw::c_char>>() - 8usize];
    [
        "Align of template specialization: test_MixedUnionPointer_open0_char_int_close0",
    ][::std::mem::align_of::<test_MixedUnionPointer<::std::os::raw::c_char>>() - 8usize];
};
