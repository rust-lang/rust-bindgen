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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ConstPtrMutObj>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ConstPtrMutObj),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MutPtrMutObj>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MutPtrMutObj),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MutPtrConstObj>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MutPtrConstObj),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ConstPtrConstObj>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ConstPtrConstObj),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
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
