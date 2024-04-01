#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ClassA {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassA_ClassAInner<T> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub x: *mut T,
}
impl<T> Default for ClassA_ClassAInner<T> {
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
pub struct ClassB {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ClassC {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassC_ClassCInnerB {
    pub cache: *mut ClassC_ClassCInnerA,
}
impl Default for ClassC_ClassCInnerB {
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
pub struct ClassC_ClassCInnerA {
    pub member: *mut ClassC_ClassCInnerB,
}
impl Default for ClassC_ClassCInnerA {
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
pub struct ClassC_ClassCInnerCRTP {
    pub _address: u8,
}
impl Default for ClassC_ClassCInnerCRTP {
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
pub struct ClassD {
    pub _address: u8,
}
const _: () = {
    ["Size of ClassD"][::std::mem::size_of::<ClassD>() - 1usize];
    ["Alignment of ClassD"][::std::mem::align_of::<ClassD>() - 1usize];
};
impl Default for ClassD {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: ClassB_open0_ClassD_ClassCInnerCRTP_close0",
    ][::std::mem::size_of::<ClassB>() - 1usize];
    [
        "Align of template specialization: ClassB_open0_ClassD_ClassCInnerCRTP_close0",
    ][::std::mem::align_of::<ClassB>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassCInnerCRTP {
    pub _address: u8,
}
const _: () = {
    ["Size of ClassCInnerCRTP"][::std::mem::size_of::<ClassCInnerCRTP>() - 1usize];
    ["Alignment of ClassCInnerCRTP"][::std::mem::align_of::<ClassCInnerCRTP>() - 1usize];
};
impl Default for ClassCInnerCRTP {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
const _: () = {
    [
        "Size of template specialization: ClassB_open0_ClassCInnerCRTP_ClassAInner_close0",
    ][::std::mem::size_of::<ClassB>() - 1usize];
    [
        "Align of template specialization: ClassB_open0_ClassCInnerCRTP_ClassAInner_close0",
    ][::std::mem::align_of::<ClassB>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClassAInner {
    pub x: *mut ClassCInnerA,
}
const _: () = {
    ["Size of ClassAInner"][::std::mem::size_of::<ClassAInner>() - 8usize];
    ["Alignment of ClassAInner"][::std::mem::align_of::<ClassAInner>() - 8usize];
    ["Offset of field: ClassAInner::x"][::std::mem::offset_of!(ClassAInner, x) - 0usize];
};
impl Default for ClassAInner {
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
pub struct ClassCInnerA {
    pub member: *mut ClassCInnerB,
}
const _: () = {
    ["Size of ClassCInnerA"][::std::mem::size_of::<ClassCInnerA>() - 8usize];
    ["Alignment of ClassCInnerA"][::std::mem::align_of::<ClassCInnerA>() - 8usize];
    [
        "Offset of field: ClassCInnerA::member",
    ][::std::mem::offset_of!(ClassCInnerA, member) - 0usize];
};
impl Default for ClassCInnerA {
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
pub struct ClassCInnerB {
    pub cache: *mut ClassCInnerA,
}
const _: () = {
    ["Size of ClassCInnerB"][::std::mem::size_of::<ClassCInnerB>() - 8usize];
    ["Alignment of ClassCInnerB"][::std::mem::align_of::<ClassCInnerB>() - 8usize];
    [
        "Offset of field: ClassCInnerB::cache",
    ][::std::mem::offset_of!(ClassCInnerB, cache) - 0usize];
};
impl Default for ClassCInnerB {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
