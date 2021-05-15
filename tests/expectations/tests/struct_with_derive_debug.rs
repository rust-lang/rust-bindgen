#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct LittleArray {
    pub a: [::std::os::raw::c_int; 32usize],
}
#[test]
fn bindgen_test_layout_LittleArray() {
    assert_eq!(
        ::std::mem::size_of::<LittleArray>(),
        128usize,
        concat!("Size of: ", stringify!(LittleArray))
    );
    assert_eq!(
        ::std::mem::align_of::<LittleArray>(),
        4usize,
        concat!("Alignment of ", stringify!(LittleArray))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<LittleArray>() };
            let struct_ptr = &struct_instance as *const LittleArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(LittleArray),
            "::",
            stringify!(a)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BigArray {
    pub a: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_BigArray() {
    assert_eq!(
        ::std::mem::size_of::<BigArray>(),
        132usize,
        concat!("Size of: ", stringify!(BigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<BigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(BigArray))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<BigArray>() };
            let struct_ptr = &struct_instance as *const BigArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BigArray),
            "::",
            stringify!(a)
        )
    );
}
impl Default for BigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct WithLittleArray {
    pub a: LittleArray,
}
#[test]
fn bindgen_test_layout_WithLittleArray() {
    assert_eq!(
        ::std::mem::size_of::<WithLittleArray>(),
        128usize,
        concat!("Size of: ", stringify!(WithLittleArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithLittleArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithLittleArray))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<WithLittleArray>() };
            let struct_ptr = &struct_instance as *const WithLittleArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithLittleArray),
            "::",
            stringify!(a)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithBigArray {
    pub a: BigArray,
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(
        ::std::mem::size_of::<WithBigArray>(),
        132usize,
        concat!("Size of: ", stringify!(WithBigArray))
    );
    assert_eq!(
        ::std::mem::align_of::<WithBigArray>(),
        4usize,
        concat!("Alignment of ", stringify!(WithBigArray))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<WithBigArray>() };
            let struct_ptr = &struct_instance as *const WithBigArray;
            let field_ptr = std::ptr::addr_of!(struct_instance.a);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WithBigArray),
            "::",
            stringify!(a)
        )
    );
}
impl Default for WithBigArray {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
