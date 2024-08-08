#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// If Bindgen could only determine the size and alignment of a
/// type, it is represented like this.
#[derive(PartialEq, Copy, Clone, Hash, Debug)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T, const N: usize>(pub [T; N]);
/// `Default` is only implemented for zero-sized opaque types, since
/// Bindgen does not know what contents make sense as a default.
impl<T> Default for __BindgenOpaqueArray<T, 0> {
    fn default() -> Self {
        Self([])
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub member: *mut __BindgenOpaqueArray<u8, 0usize>,
}
pub type Foo_FirstAlias = __BindgenOpaqueArray<u8, 0usize>;
pub type Foo_SecondAlias = __BindgenOpaqueArray<u8, 0usize>;
impl Default for Foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
