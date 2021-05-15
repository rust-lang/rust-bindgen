#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Blocklisted<T> {
    t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}

/// This would derive(Hash, Eq, PartialEq) if it didn't contain a blocklisted type,
/// causing us to conservatively avoid deriving hash/Eq/PartialEq for it.
#[repr(C)]
pub struct AllowlistedOne {
    pub a: Blocklisted<::std::os::raw::c_int>,
}
#[test]
fn bindgen_test_layout_AllowlistedOne() {
    assert_eq!(
        ::std::mem::size_of::<AllowlistedOne>(),
        4usize,
        concat!("Size of: ", stringify!(AllowlistedOne))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistedOne>(),
        4usize,
        concat!("Alignment of ", stringify!(AllowlistedOne))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<AllowlistedOne>() };
            let struct_ptr = &struct_instance as *const AllowlistedOne;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllowlistedOne),
            "::",
            stringify!(a)
        )
    );
}
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
#[test]
fn bindgen_test_layout_AllowlistedTwo() {
    assert_eq!(
        ::std::mem::size_of::<AllowlistedTwo>(),
        4usize,
        concat!("Size of: ", stringify!(AllowlistedTwo))
    );
    assert_eq!(
        ::std::mem::align_of::<AllowlistedTwo>(),
        4usize,
        concat!("Alignment of ", stringify!(AllowlistedTwo))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<AllowlistedTwo>() };
            let struct_ptr = &struct_instance as *const AllowlistedTwo;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(AllowlistedTwo),
            "::",
            stringify!(b)
        )
    );
}
impl Default for AllowlistedTwo {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
