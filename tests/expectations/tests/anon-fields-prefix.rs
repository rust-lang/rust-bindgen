#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone)]
pub union color {
    pub u1: color__bindgen_ty_1,
    pub u2: color__bindgen_ty_2,
    pub v3: [::std::os::raw::c_uchar; 3usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_1 {
    pub r: ::std::os::raw::c_uchar,
    pub g: ::std::os::raw::c_uchar,
    pub b: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_color__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<color__bindgen_ty_1>(),
        3usize,
        concat!("Size of: ", stringify!(color__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<color__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(color__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.r);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.g);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(g)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct color__bindgen_ty_2 {
    pub y: ::std::os::raw::c_uchar,
    pub u: ::std::os::raw::c_uchar,
    pub v: ::std::os::raw::c_uchar,
}
#[test]
fn bindgen_test_layout_color__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<color__bindgen_ty_2>(),
        3usize,
        concat!("Size of: ", stringify!(color__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<color__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(color__bindgen_ty_2))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_2>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.y);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_2>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.u);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<color__bindgen_ty_2>() };
            let struct_ptr = &struct_instance as *const color__bindgen_ty_2;
            let field_ptr = std::ptr::addr_of!(struct_instance.v);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(color__bindgen_ty_2),
            "::",
            stringify!(v)
        )
    );
}
#[test]
fn bindgen_test_layout_color() {
    assert_eq!(
        ::std::mem::size_of::<color>(),
        3usize,
        concat!("Size of: ", stringify!(color))
    );
    assert_eq!(
        ::std::mem::align_of::<color>(),
        1usize,
        concat!("Alignment of ", stringify!(color))
    );
}
impl Default for color {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
