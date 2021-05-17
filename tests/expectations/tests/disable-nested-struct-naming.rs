#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub b1: bar1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1 {
    pub x1: ::std::os::raw::c_int,
    pub b2: bar1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1 {
    pub x2: ::std::os::raw::c_int,
    pub b3: bar1__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1__bindgen_ty_1 {
    pub x3: ::std::os::raw::c_int,
    pub b4: bar4,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar4 {
    pub x4: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_bar4() {
    assert_eq!(
        ::std::mem::size_of::<bar4>(),
        4usize,
        concat!("Size of: ", stringify!(bar4))
    );
    assert_eq!(
        ::std::mem::align_of::<bar4>(),
        4usize,
        concat!("Alignment of ", stringify!(bar4))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<bar4>() };
            let struct_ptr = &struct_instance as *const bar4;
            let field_ptr = std::ptr::addr_of!(struct_instance.x4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(bar4), "::", stringify!(x4))
    );
}
#[test]
fn bindgen_test_layout_bar1__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bar1__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(bar1__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(bar1__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<bar1__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr =
                &struct_instance as *const bar1__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.x3);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar1__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(x3)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe {
                std::mem::zeroed::<bar1__bindgen_ty_1__bindgen_ty_1>()
            };
            let struct_ptr =
                &struct_instance as *const bar1__bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b4);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bar1__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(b4)
        )
    );
}
#[test]
fn bindgen_test_layout_bar1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bar1__bindgen_ty_1>(),
        12usize,
        concat!("Size of: ", stringify!(bar1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(bar1__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<bar1__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const bar1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.x2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(bar1__bindgen_ty_1),
            "::",
            stringify!(x2)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<bar1__bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const bar1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b3);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(bar1__bindgen_ty_1),
            "::",
            stringify!(b3)
        )
    );
}
#[test]
fn bindgen_test_layout_bar1() {
    assert_eq!(
        ::std::mem::size_of::<bar1>(),
        16usize,
        concat!("Size of: ", stringify!(bar1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1>(),
        4usize,
        concat!("Alignment of ", stringify!(bar1))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<bar1>() };
            let struct_ptr = &struct_instance as *const bar1;
            let field_ptr = std::ptr::addr_of!(struct_instance.x1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(bar1), "::", stringify!(x1))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<bar1>() };
            let struct_ptr = &struct_instance as *const bar1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(bar1), "::", stringify!(b2))
    );
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<foo>() };
            let struct_ptr = &struct_instance as *const foo;
            let field_ptr = std::ptr::addr_of!(struct_instance.b1);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(foo), "::", stringify!(b1))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub anon2: _bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1__bindgen_ty_1 {
    pub b: baz,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_baz() {
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        4usize,
        concat!("Size of: ", stringify!(baz))
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        4usize,
        concat!("Alignment of ", stringify!(baz))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<baz>() };
            let struct_ptr = &struct_instance as *const baz;
            let field_ptr = std::ptr::addr_of!(struct_instance.x);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(baz), "::", stringify!(x))
    );
}
#[test]
fn bindgen_test_layout__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<_bindgen_ty_1__bindgen_ty_1>() };
            let struct_ptr =
                &struct_instance as *const _bindgen_ty_1__bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(b)
        )
    );
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<_bindgen_ty_1>() };
            let struct_ptr = &struct_instance as *const _bindgen_ty_1;
            let field_ptr = std::ptr::addr_of!(struct_instance.anon2);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_bindgen_ty_1),
            "::",
            stringify!(anon2)
        )
    );
}
extern "C" {
    pub static mut anon1: _bindgen_ty_1;
}
