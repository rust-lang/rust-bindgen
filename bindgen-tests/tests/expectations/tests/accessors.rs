#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SomeAccessors"][::std::mem::size_of::<SomeAccessors>() - 16usize];
    ["Alignment of SomeAccessors"][::std::mem::align_of::<SomeAccessors>() - 4usize];
    [
        "Offset of field: SomeAccessors::mNoAccessor",
    ][::std::mem::offset_of!(SomeAccessors, mNoAccessor) - 0usize];
    [
        "Offset of field: SomeAccessors::mBothAccessors",
    ][::std::mem::offset_of!(SomeAccessors, mBothAccessors) - 4usize];
    [
        "Offset of field: SomeAccessors::mUnsafeAccessors",
    ][::std::mem::offset_of!(SomeAccessors, mUnsafeAccessors) - 8usize];
    [
        "Offset of field: SomeAccessors::mImmutableAccessor",
    ][::std::mem::offset_of!(SomeAccessors, mImmutableAccessor) - 12usize];
};
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
    pub unsafe fn get_mUnsafeAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AllAccessors"][::std::mem::size_of::<AllAccessors>() - 8usize];
    ["Alignment of AllAccessors"][::std::mem::align_of::<AllAccessors>() - 4usize];
    [
        "Offset of field: AllAccessors::mBothAccessors",
    ][::std::mem::offset_of!(AllAccessors, mBothAccessors) - 0usize];
    [
        "Offset of field: AllAccessors::mAlsoBothAccessors",
    ][::std::mem::offset_of!(AllAccessors, mAlsoBothAccessors) - 4usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of AllUnsafeAccessors"][::std::mem::size_of::<AllUnsafeAccessors>() - 8usize];
    [
        "Alignment of AllUnsafeAccessors",
    ][::std::mem::align_of::<AllUnsafeAccessors>() - 4usize];
    [
        "Offset of field: AllUnsafeAccessors::mBothAccessors",
    ][::std::mem::offset_of!(AllUnsafeAccessors, mBothAccessors) - 0usize];
    [
        "Offset of field: AllUnsafeAccessors::mAlsoBothAccessors",
    ][::std::mem::offset_of!(AllUnsafeAccessors, mAlsoBothAccessors) - 4usize];
};
impl AllUnsafeAccessors {
    #[inline]
    pub unsafe fn get_mBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
        &mut self.mBothAccessors
    }
    #[inline]
    pub unsafe fn get_mAlsoBothAccessors(&self) -> &::std::os::raw::c_int {
        &self.mAlsoBothAccessors
    }
    #[inline]
    pub unsafe fn get_mAlsoBothAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ContradictAccessors",
    ][::std::mem::size_of::<ContradictAccessors>() - 16usize];
    [
        "Alignment of ContradictAccessors",
    ][::std::mem::align_of::<ContradictAccessors>() - 4usize];
    [
        "Offset of field: ContradictAccessors::mBothAccessors",
    ][::std::mem::offset_of!(ContradictAccessors, mBothAccessors) - 0usize];
    [
        "Offset of field: ContradictAccessors::mNoAccessors",
    ][::std::mem::offset_of!(ContradictAccessors, mNoAccessors) - 4usize];
    [
        "Offset of field: ContradictAccessors::mUnsafeAccessors",
    ][::std::mem::offset_of!(ContradictAccessors, mUnsafeAccessors) - 8usize];
    [
        "Offset of field: ContradictAccessors::mImmutableAccessor",
    ][::std::mem::offset_of!(ContradictAccessors, mImmutableAccessor) - 12usize];
};
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
    pub unsafe fn get_mUnsafeAccessors_mut(&mut self) -> &mut ::std::os::raw::c_int {
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Replaced"][::std::mem::size_of::<Replaced>() - 4usize];
    ["Alignment of Replaced"][::std::mem::align_of::<Replaced>() - 4usize];
    [
        "Offset of field: Replaced::mAccessor",
    ][::std::mem::offset_of!(Replaced, mAccessor) - 0usize];
};
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Wrapper"][::std::mem::size_of::<Wrapper>() - 4usize];
    ["Alignment of Wrapper"][::std::mem::align_of::<Wrapper>() - 4usize];
    [
        "Offset of field: Wrapper::mReplaced",
    ][::std::mem::offset_of!(Wrapper, mReplaced) - 0usize];
};
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
