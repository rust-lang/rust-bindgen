#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Pointers can derive Hash/PartialOrd/Ord/PartialEq/Eq
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConstPtrMutObj {
    pub bar: *mut ::std::os::raw::c_int,
}
const _: () = {
    ["Size of ConstPtrMutObj"][::std::mem::size_of::<ConstPtrMutObj>() - 8usize];
    ["Alignment of ConstPtrMutObj"][::std::mem::align_of::<ConstPtrMutObj>() - 8usize];
    [
        "Offset of field: ConstPtrMutObj::bar",
    ][::std::mem::offset_of!(ConstPtrMutObj, bar) - 0usize];
};
impl Default for ConstPtrMutObj {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct MutPtrMutObj {
    pub bar: *mut ::std::os::raw::c_int,
}
const _: () = {
    ["Size of MutPtrMutObj"][::std::mem::size_of::<MutPtrMutObj>() - 8usize];
    ["Alignment of MutPtrMutObj"][::std::mem::align_of::<MutPtrMutObj>() - 8usize];
    [
        "Offset of field: MutPtrMutObj::bar",
    ][::std::mem::offset_of!(MutPtrMutObj, bar) - 0usize];
};
impl Default for MutPtrMutObj {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct MutPtrConstObj {
    pub bar: *const ::std::os::raw::c_int,
}
const _: () = {
    ["Size of MutPtrConstObj"][::std::mem::size_of::<MutPtrConstObj>() - 8usize];
    ["Alignment of MutPtrConstObj"][::std::mem::align_of::<MutPtrConstObj>() - 8usize];
    [
        "Offset of field: MutPtrConstObj::bar",
    ][::std::mem::offset_of!(MutPtrConstObj, bar) - 0usize];
};
impl Default for MutPtrConstObj {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConstPtrConstObj {
    pub bar: *const ::std::os::raw::c_int,
}
const _: () = {
    ["Size of ConstPtrConstObj"][::std::mem::size_of::<ConstPtrConstObj>() - 8usize];
    [
        "Alignment of ConstPtrConstObj",
    ][::std::mem::align_of::<ConstPtrConstObj>() - 8usize];
    [
        "Offset of field: ConstPtrConstObj::bar",
    ][::std::mem::offset_of!(ConstPtrConstObj, bar) - 0usize];
};
impl Default for ConstPtrConstObj {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
