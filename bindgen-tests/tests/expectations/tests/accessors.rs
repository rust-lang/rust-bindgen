#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SomeAccessors {
    pub mNoAccessor: ::std::os::raw::c_int,
    /// <div rustbindgen accessor></div>
    pub mBothAccessors: ::std::os::raw::c_int,
    /// <div rustbindgen accessor="unsafe"></div>
    pub mUnsafeAccessors: ::std::os::raw::c_int,
    /// <div rustbindgen accessor="immutable"></div>
    pub mImmutableAccessor: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_SomeAccessors() {
    const UNINIT: ::std::mem::MaybeUninit<SomeAccessors> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SomeAccessors>(),
        16usize,
        concat!("Size of: ", stringify!(SomeAccessors))
    );
    assert_eq!(
        ::std::mem::align_of::<SomeAccessors>(),
        4usize,
        concat!("Alignment of ", stringify!(SomeAccessors))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mNoAccessor) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SomeAccessors),
            "::",
            stringify!(mNoAccessor)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mBothAccessors) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(SomeAccessors),
            "::",
            stringify!(mBothAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mUnsafeAccessors) as usize -
                ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SomeAccessors),
            "::",
            stringify!(mUnsafeAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mImmutableAccessor) as usize -
                ptr as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(SomeAccessors),
            "::",
            stringify!(mImmutableAccessor)
        )
    );
}
impl SomeAccessors {
    #[inline]
    pub fn get_mBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mBothAccessors
    }
    #[inline]
    pub fn get_mBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mUnsafeAccessors(&self) -> &::std::os::raw::c_int {
        &self.mUnsafeAccessors
    }
    #[inline]
    pub unsafe fn get_mUnsafeAccessors_mut(
        &mut self,
    ) -> &mut ::std::os::raw::c_int {
        &mut self.mUnsafeAccessors
    }
    #[inline]
    pub fn get_mImmutableAccessor(&self) -> &::std::os::raw::c_int {
        &self.mImmutableAccessor
    }
}
/// <div rustbindgen accessor></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllAccessors {
    pub mBothAccessors: ::std::os::raw::c_int,
    pub mAlsoBothAccessors: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AllAccessors() {
    const UNINIT: ::std::mem::MaybeUninit<AllAccessors> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AllAccessors>(),
        8usize,
        concat!("Size of: ", stringify!(AllAccessors))
    );
    assert_eq!(
        ::std::mem::align_of::<AllAccessors>(),
        4usize,
        concat!("Alignment of ", stringify!(AllAccessors))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mBothAccessors) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllAccessors),
            "::",
            stringify!(mBothAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mAlsoBothAccessors) as usize -
                ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AllAccessors),
            "::",
            stringify!(mAlsoBothAccessors)
        )
    );
}
impl AllAccessors {
    #[inline]
    pub fn get_mBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mBothAccessors
    }
    #[inline]
    pub fn get_mBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mBothAccessors
    }
    #[inline]
    pub fn get_mAlsoBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mAlsoBothAccessors
    }
    #[inline]
    pub fn get_mAlsoBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mAlsoBothAccessors
    }
}
/// <div rustbindgen accessor="unsafe"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AllUnsafeAccessors {
    pub mBothAccessors: ::std::os::raw::c_int,
    pub mAlsoBothAccessors: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_AllUnsafeAccessors() {
    const UNINIT: ::std::mem::MaybeUninit<AllUnsafeAccessors> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<AllUnsafeAccessors>(),
        8usize,
        concat!("Size of: ", stringify!(AllUnsafeAccessors))
    );
    assert_eq!(
        ::std::mem::align_of::<AllUnsafeAccessors>(),
        4usize,
        concat!("Alignment of ", stringify!(AllUnsafeAccessors))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mBothAccessors) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllUnsafeAccessors),
            "::",
            stringify!(mBothAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mAlsoBothAccessors) as usize -
                ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(AllUnsafeAccessors),
            "::",
            stringify!(mAlsoBothAccessors)
        )
    );
}
impl AllUnsafeAccessors {
    #[inline]
    pub unsafe fn get_mBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mBothAccessors_mut(
        &mut self,
    ) -> &mut ::std::os::raw::c_int {
        &mut self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mAlsoBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mAlsoBothAccessors
    }
    #[inline]
    pub unsafe fn get_mAlsoBothAccessors_mut(
        &mut self,
    ) -> &mut ::std::os::raw::c_int {
        &mut self.mAlsoBothAccessors
    }
}
/// <div rustbindgen accessor></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ContradictAccessors {
    pub mBothAccessors: ::std::os::raw::c_int,
    /// <div rustbindgen accessor="false"></div>
    pub mNoAccessors: ::std::os::raw::c_int,
    /// <div rustbindgen accessor="unsafe"></div>
    pub mUnsafeAccessors: ::std::os::raw::c_int,
    /// <div rustbindgen accessor="immutable"></div>
    pub mImmutableAccessor: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContradictAccessors() {
    const UNINIT: ::std::mem::MaybeUninit<ContradictAccessors> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ContradictAccessors>(),
        16usize,
        concat!("Size of: ", stringify!(ContradictAccessors))
    );
    assert_eq!(
        ::std::mem::align_of::<ContradictAccessors>(),
        4usize,
        concat!("Alignment of ", stringify!(ContradictAccessors))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mBothAccessors) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictAccessors),
            "::",
            stringify!(mBothAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mNoAccessors) as usize - ptr as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictAccessors),
            "::",
            stringify!(mNoAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mUnsafeAccessors) as usize -
                ptr as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictAccessors),
            "::",
            stringify!(mUnsafeAccessors)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mImmutableAccessor) as usize -
                ptr as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictAccessors),
            "::",
            stringify!(mImmutableAccessor)
        )
    );
}
impl ContradictAccessors {
    #[inline]
    pub fn get_mBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mBothAccessors
    }
    #[inline]
    pub fn get_mBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mUnsafeAccessors(&self) -> &::std::os::raw::c_int {
        &self.mUnsafeAccessors
    }
    #[inline]
    pub unsafe fn get_mUnsafeAccessors_mut(
        &mut self,
    ) -> &mut ::std::os::raw::c_int {
        &mut self.mUnsafeAccessors
    }
    #[inline]
    pub fn get_mImmutableAccessor(&self) -> &::std::os::raw::c_int {
        &self.mImmutableAccessor
    }
}
/// <div rustbindgen accessor replaces="Replaced"></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Replaced {
    pub mAccessor: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Replaced() {
    const UNINIT: ::std::mem::MaybeUninit<Replaced> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Replaced>(),
        4usize,
        concat!("Size of: ", stringify!(Replaced))
    );
    assert_eq!(
        ::std::mem::align_of::<Replaced>(),
        4usize,
        concat!("Alignment of ", stringify!(Replaced))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mAccessor) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Replaced),
            "::",
            stringify!(mAccessor)
        )
    );
}
impl Replaced {
    #[inline]
    pub fn get_mAccessor(&self) -> &::std::os::raw::c_int {
        &self.mAccessor
    }
    #[inline]
    pub fn get_mAccessor_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mAccessor
    }
}
/// <div rustbindgen accessor></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Wrapper {
    pub mReplaced: Replaced,
}
#[test]
fn bindgen_test_layout_Wrapper() {
    const UNINIT: ::std::mem::MaybeUninit<Wrapper> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Wrapper>(),
        4usize,
        concat!("Size of: ", stringify!(Wrapper))
    );
    assert_eq!(
        ::std::mem::align_of::<Wrapper>(),
        4usize,
        concat!("Alignment of ", stringify!(Wrapper))
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).mReplaced) as usize - ptr as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Wrapper),
            "::",
            stringify!(mReplaced)
        )
    );
}
impl Wrapper {
    #[inline]
    pub fn get_mReplaced(&self) -> &Replaced {
        &self.mReplaced
    }
    #[inline]
    pub fn get_mReplaced_mut(&mut self) -> &mut Replaced {
        &mut self.mReplaced
    }
}
