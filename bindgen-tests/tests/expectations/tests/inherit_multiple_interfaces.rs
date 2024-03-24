#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
pub struct A__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A {
    pub vtable_: *const A__bindgen_vtable,
    pub member: ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<A>() == 16usize, "Size of A");
    assert!(::std::mem::align_of::<A>() == 8usize, "Alignment of A");
    assert!(::std::mem::offset_of!(A, member) == 8usize, "Offset of field: A::member");
};
impl Default for A {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct B__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct B {
    pub vtable_: *const B__bindgen_vtable,
    pub member2: *mut ::std::os::raw::c_void,
}
const _: () = {
    assert!(::std::mem::size_of::<B>() == 16usize, "Size of B");
    assert!(::std::mem::align_of::<B>() == 8usize, "Alignment of B");
    assert!(::std::mem::offset_of!(B, member2) == 8usize, "Offset of field: B::member2");
};
impl Default for B {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct C {
    pub _base: A,
    pub _base_1: B,
    pub member3: f32,
}
const _: () = {
    assert!(::std::mem::size_of::<C>() == 40usize, "Size of C");
    assert!(::std::mem::align_of::<C>() == 8usize, "Alignment of C");
    assert!(
        ::std::mem::offset_of!(C, member3) == 32usize,
        "Offset of field: C::member3",
    );
};
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
