#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// Pointers can derive Hash/PartialOrd/Ord/PartialEq/Eq
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConstPtrMutObj {
    pub bar: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ConstPtrMutObj() {
    const UNINIT: ::std::mem::MaybeUninit<ConstPtrMutObj> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ConstPtrMutObj>(),
        8usize,
        concat!("Size of: ", stringify!(ConstPtrMutObj))
    );
    assert_eq!(
        ::std::mem::align_of::<ConstPtrMutObj>(),
        8usize,
        concat!("Alignment of ", stringify!(ConstPtrMutObj))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConstPtrMutObj),
            "::",
            stringify!(bar)
        )
    );
}
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
#[test]
fn bindgen_test_layout_MutPtrMutObj() {
    const UNINIT: ::std::mem::MaybeUninit<MutPtrMutObj> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<MutPtrMutObj>(),
        8usize,
        concat!("Size of: ", stringify!(MutPtrMutObj))
    );
    assert_eq!(
        ::std::mem::align_of::<MutPtrMutObj>(),
        8usize,
        concat!("Alignment of ", stringify!(MutPtrMutObj))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MutPtrMutObj),
            "::",
            stringify!(bar)
        )
    );
}
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
#[test]
fn bindgen_test_layout_MutPtrConstObj() {
    const UNINIT: ::std::mem::MaybeUninit<MutPtrConstObj> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<MutPtrConstObj>(),
        8usize,
        concat!("Size of: ", stringify!(MutPtrConstObj))
    );
    assert_eq!(
        ::std::mem::align_of::<MutPtrConstObj>(),
        8usize,
        concat!("Alignment of ", stringify!(MutPtrConstObj))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MutPtrConstObj),
            "::",
            stringify!(bar)
        )
    );
}
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
#[test]
fn bindgen_test_layout_ConstPtrConstObj() {
    const UNINIT: ::std::mem::MaybeUninit<ConstPtrConstObj> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ConstPtrConstObj>(),
        8usize,
        concat!("Size of: ", stringify!(ConstPtrConstObj))
    );
    assert_eq!(
        ::std::mem::align_of::<ConstPtrConstObj>(),
        8usize,
        concat!("Alignment of ", stringify!(ConstPtrConstObj))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConstPtrConstObj),
            "::",
            stringify!(bar)
        )
    );
}
impl Default for ConstPtrConstObj {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
