#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
#[repr(C)]
pub struct __BindgenOpaqueArray<T>(pub T);
impl<T: Copy + Default, const N: usize> Default for __BindgenOpaqueArray<[T; N]> {
    fn default() -> Self {
        Self([<T as Default>::default(); N])
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub member: *mut __BindgenOpaqueArray<[u8; 0usize]>,
}
pub type Foo_FirstAlias = __BindgenOpaqueArray<[u8; 0usize]>;
pub type Foo_SecondAlias = __BindgenOpaqueArray<[u8; 0usize]>;
impl Default for Foo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
