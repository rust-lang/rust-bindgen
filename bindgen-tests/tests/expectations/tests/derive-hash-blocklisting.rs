#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[repr(C)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Blocklisted<T> {
    t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
/** This would derive(Hash, Eq, PartialEq) if it didn't contain a blocklisted type,
 causing us to conservatively avoid deriving hash/Eq/PartialEq for it.*/
#[repr(C)]
pub struct AllowlistedOne {
    pub a: Blocklisted<::std::os::raw::c_int>,
}
const _: () = {
    assert!(::std::mem::size_of::<AllowlistedOne>() == 4usize, "Size of AllowlistedOne");
    assert!(
        ::std::mem::align_of::<AllowlistedOne>() == 4usize,
        "Alignment of AllowlistedOne",
    );
    assert!(
        ::std::mem::offset_of!(AllowlistedOne, a) == 0usize,
        "Offset of field: AllowlistedOne::a",
    );
};
impl Default for AllowlistedOne {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
/// This can't derive(Hash/Eq) even if it didn't contain a blocklisted type.
#[repr(C)]
pub struct AllowlistedTwo {
    pub b: Blocklisted<f32>,
}
const _: () = {
    assert!(::std::mem::size_of::<AllowlistedTwo>() == 4usize, "Size of AllowlistedTwo");
    assert!(
        ::std::mem::align_of::<AllowlistedTwo>() == 4usize,
        "Alignment of AllowlistedTwo",
    );
    assert!(
        ::std::mem::offset_of!(AllowlistedTwo, b) == 0usize,
        "Offset of field: AllowlistedTwo::b",
    );
};
impl Default for AllowlistedTwo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
