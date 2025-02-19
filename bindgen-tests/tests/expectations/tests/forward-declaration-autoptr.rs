#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug)]
pub struct Foo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RefPtr<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub m_inner: *mut T,
}
impl<T> Default for RefPtr<T> {
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
pub struct Bar {
    pub m_member: RefPtr<Foo>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Bar"][::std::mem::size_of::<Bar>() - 8usize];
    ["Alignment of Bar"][::std::mem::align_of::<Bar>() - 8usize];
    ["Offset of field: Bar::m_member"][::std::mem::offset_of!(Bar, m_member) - 0usize];
};
impl Default for Bar {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of template specialization: RefPtr_open0_Foo_close0",
    ][::std::mem::size_of::<RefPtr<Foo>>() - 8usize];
    [
        "Align of template specialization: RefPtr_open0_Foo_close0",
    ][::std::mem::align_of::<RefPtr<Foo>>() - 8usize];
};
