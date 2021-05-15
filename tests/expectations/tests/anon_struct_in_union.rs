#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct s {
    pub u: s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union s__bindgen_ty_1 {
    pub field: s__bindgen_ty_1_inner,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct s__bindgen_ty_1_inner {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1_inner() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1_inner>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1_inner))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<s__bindgen_ty_1_inner>() };
            let struct_ptr = &struct_instance as *const s__bindgen_ty_1_inner;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(s__bindgen_ty_1_inner),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout_s__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(s__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<s__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(s__bindgen_ty_1))
    );
}
impl Default for s__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_s() {
    assert_eq!(
        ::std::mem::size_of::<s>(),
        4usize,
        concat!("Size of: ", stringify!(s))
    );
    assert_eq!(
        ::std::mem::align_of::<s>(),
        4usize,
        concat!("Alignment of ", stringify!(s))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<s>() };
            let struct_ptr = &struct_instance as *const s;
            let field_ptr = std::ptr::addr_of!(struct_instance.u);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(s), "::", stringify!(u))
    );
}
impl Default for s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
