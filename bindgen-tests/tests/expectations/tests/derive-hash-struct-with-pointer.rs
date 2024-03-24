#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
/// Pointers can derive Hash/PartialOrd/Ord/PartialEq/Eq
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConstPtrMutObj {
    pub bar: *mut ::std::os::raw::c_int,
}
const _: () = {
    assert!(::std::mem::size_of::<ConstPtrMutObj>() == 8usize, "Size of ConstPtrMutObj");
    assert!(
        ::std::mem::align_of::<ConstPtrMutObj>() == 8usize,
        "Alignment of ConstPtrMutObj",
    );
    assert!(
        ::std::mem::offset_of!(ConstPtrMutObj, bar) == 0usize,
        "Offset of field: ConstPtrMutObj::bar",
    );
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
    assert!(::std::mem::size_of::<MutPtrMutObj>() == 8usize, "Size of MutPtrMutObj");
    assert!(
        ::std::mem::align_of::<MutPtrMutObj>() == 8usize,
        "Alignment of MutPtrMutObj",
    );
    assert!(
        ::std::mem::offset_of!(MutPtrMutObj, bar) == 0usize,
        "Offset of field: MutPtrMutObj::bar",
    );
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
    assert!(::std::mem::size_of::<MutPtrConstObj>() == 8usize, "Size of MutPtrConstObj");
    assert!(
        ::std::mem::align_of::<MutPtrConstObj>() == 8usize,
        "Alignment of MutPtrConstObj",
    );
    assert!(
        ::std::mem::offset_of!(MutPtrConstObj, bar) == 0usize,
        "Offset of field: MutPtrConstObj::bar",
    );
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
    assert!(
        ::std::mem::size_of::<ConstPtrConstObj>() == 8usize,
        "Size of ConstPtrConstObj",
    );
    assert!(
        ::std::mem::align_of::<ConstPtrConstObj>() == 8usize,
        "Alignment of ConstPtrConstObj",
    );
    assert!(
        ::std::mem::offset_of!(ConstPtrConstObj, bar) == 0usize,
        "Offset of field: ConstPtrConstObj::bar",
    );
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
