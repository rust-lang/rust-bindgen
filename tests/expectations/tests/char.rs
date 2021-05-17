#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type Char = ::std::os::raw::c_char;
pub type SChar = ::std::os::raw::c_schar;
pub type UChar = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Test {
    pub ch: ::std::os::raw::c_char,
    pub u: ::std::os::raw::c_uchar,
    pub d: ::std::os::raw::c_schar,
    pub cch: ::std::os::raw::c_char,
    pub cu: ::std::os::raw::c_uchar,
    pub cd: ::std::os::raw::c_schar,
    pub Cch: Char,
    pub Cu: UChar,
    pub Cd: SChar,
    pub Ccch: Char,
    pub Ccu: UChar,
    pub Ccd: SChar,
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        12usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        1usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.ch);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(ch))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.u);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        1usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(u))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        2usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(d))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.cch);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        3usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cch))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.cu);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cu))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.cd);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        5usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(cd))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Cch);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        6usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cch))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Cu);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        7usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cu))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Cd);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        8usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Cd))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Ccch);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccch)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Ccu);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        10usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccu))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<Test>() };
            let struct_ptr = &struct_instance as *const Test;
            let field_ptr = std::ptr::addr_of!(struct_instance.Ccd);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        11usize,
        concat!("Offset of field: ", stringify!(Test), "::", stringify!(Ccd))
    );
}
